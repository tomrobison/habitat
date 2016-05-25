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
use rustc_serialize::json;

use error::{Error, Result};

/// Contains metadata entries for each package known by the Depot
pub struct PackagesTable {
    pub index: PackagesIndex,
    pool: Arc<ConnectionPool>,
}

impl PackagesTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let index = PackagesIndex::new(pool1);
        PackagesTable {
            pool: pool,
            index: index,
        }
    }

    pub fn get<T: AsRef<package::PackageIdent>>(&self, id: T) -> Result<data_object::Package> {
        let conn = self.pool().get().unwrap();
        match conn.get::<String, String>(Self::key(&id.as_ref().to_string())) {
            Ok(body) => {
                let pkg: data_object::Package = json::decode(&body).unwrap();
                Ok(pkg)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn write(&self, record: &data_object::Package) -> Result<()> {
        let conn = self.pool().get().unwrap();
        let keys = [Self::key(&record.ident.to_string()),
                    PackagesIndex::key(&record.ident.origin_idx()),
                    PackagesIndex::key(&record.ident.name_idx()),
                    PackagesIndex::key(&record.ident.version_idx().as_ref().unwrap())];
        try!(redis::transaction(conn.deref(), &keys, |txn| {
            let body = json::encode(&record).unwrap();
            txn.set(Self::key(&record.ident.to_string()), body)
               .ignore()
               .sadd(PackagesIndex::key(&record.ident.origin_idx()),
                     record.ident.clone())
               .ignore()
               .sadd(PackagesIndex::key(&record.ident.name_idx()),
                     record.ident.clone())
               .ignore()
               .sadd(PackagesIndex::key(&record.ident.version_idx().as_ref().unwrap()),
                     record.ident.clone())
               .ignore()
               .query(conn.deref())
        }));
        Ok(())
    }
}

impl Table for PackagesTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "package"
    }
}

/// Contains an index of package identifiers to easily find the latest version/release of a
/// specified package.
pub struct PackagesIndex {
    pool: Arc<ConnectionPool>,
}

impl PackagesIndex {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        PackagesIndex { pool: pool }
    }

    pub fn all(&self, id: &str) -> Result<Vec<package::PackageIdent>> {
        let conn = self.pool().get().unwrap();
        match conn.smembers::<String, Vec<String>>(Self::key(&id.to_string())) {
            Ok(ids) => {
                let ids = ids.iter()
                             .map(|id| package::PackageIdent::from_str(id).unwrap())
                             .collect();
                Ok(ids)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn latest<T: AsRef<package::PackageIdent>>(&self, id: T) -> Result<package::PackageIdent> {
        let conn = self.pool().get().unwrap();
        let key = PackagesIndex::key(&id.as_ref().to_string());
        match redis::cmd("SORT")
                  .arg(key)
                  .arg("LIMIT")
                  .arg(0)
                  .arg(1)
                  .arg("ALPHA")
                  .arg("DESC")
                  .query::<Vec<String>>(conn.deref()) {
            Ok(ids) => {
                if ids.is_empty() {
                    return Err(Error::DataStore(dbcache::Error::EntityNotFound));
                }
                let ident = package::PackageIdent::from_str(&ids[0]).unwrap();
                Ok(ident)
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl Table for PackagesIndex {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "package:ident:index"
    }
}

