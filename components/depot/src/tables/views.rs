// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use dbcache::{self, ConnectionPool, Table};
use depot_core::data_object::{self, DataObject};
use hcore::package;
use redis::{self, Commands, PipelineCommands};

use error::{Error, Result};

/// Contains a mapping of view names and the packages found within that view.
///
/// This is how packages will be "promoted" between environments without duplicating data on disk.
pub struct ViewsTable {
    pool: Arc<ConnectionPool>,
    pub pkg_view_idx: PkgViewIndex,
    pub view_pkg_idx: ViewPkgIndex,
}

impl ViewsTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let pkg_view_idx = PkgViewIndex::new(pool1);
        let view_pkg_idx = ViewPkgIndex::new(pool2);

        ViewsTable {
            pool: pool,
            pkg_view_idx: pkg_view_idx,
            view_pkg_idx: view_pkg_idx,
        }
    }

    pub fn all(&self) -> Result<Vec<String>> {
        let conn = self.pool.get().unwrap();
        match conn.smembers(Self::prefix()) {
            Ok(members) => Ok(members),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn associate(&self, view: &str, pkg: &data_object::Package) -> Result<()> {
        let script = redis::Script::new(r"
            redis.call('sadd', KEYS[1], ARGV[2]);
            redis.call('zadd', KEYS[2], 0, ARGV[1]);
        ");
        try!(script.arg(pkg.ident.clone())
                   .arg(view.clone())
                   .key(PkgViewIndex::key(&pkg.ident))
                   .key(ViewPkgIndex::key(&view.to_string()))
                   .invoke(self.pool.get().unwrap().deref()));
        Ok(())
    }

    pub fn is_member(&self, view: &str) -> Result<bool> {
        let conn = self.pool.get().unwrap();
        match conn.sismember(Self::prefix(), view) {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn write(&self, view: &str) -> Result<()> {
        let conn = self.pool().get().unwrap();
        try!(conn.sadd(Self::prefix(), view));
        Ok(())
    }
}

impl Table for ViewsTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "views"
    }
}

pub struct PkgViewIndex {
    pool: Arc<ConnectionPool>,
}

impl PkgViewIndex {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        PkgViewIndex { pool: pool }
    }
}

impl Table for PkgViewIndex {
    type IdType = data_object::PackageIdent;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "pkg:view:index"
    }
}


pub struct ViewPkgIndex {
    pool: Arc<ConnectionPool>,
}

impl ViewPkgIndex {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        ViewPkgIndex { pool: pool }
    }

    pub fn all(&self, view: &str, pkg: &str) -> Result<Vec<package::PackageIdent>> {
        let conn = self.pool().get().unwrap();
        match conn.zscan_match::<String, String, (String, u32)>(Self::key(&view.to_string()),
                                                                format!("{}*", pkg)) {
            Ok(set) => {
                let set: Vec<package::PackageIdent> = set.map(|(id, _)| {
                                                             package::PackageIdent::from_str(&id)
                                                                 .unwrap()
                                                         })
                                                         .collect();
                Ok(set)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn is_member<T: AsRef<package::PackageIdent>>(&self, view: &str, pkg: T) -> Result<bool> {
        let conn = self.pool().get().unwrap();
        match conn.sismember(Self::key(&view.to_string()), pkg.as_ref().to_string()) {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn latest(&self, view: &str, pkg: &str) -> Result<package::PackageIdent> {
        match self.all(view, pkg) {
            Ok(mut ids) => {
                if let Some(id) = ids.pop() {
                    Ok(id)
                } else {
                    Err(Error::DataStore(dbcache::Error::EntityNotFound))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl Table for ViewPkgIndex {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "view:pkg:index"
    }
}

