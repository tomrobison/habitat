// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::sync::Arc;

use dbcache::{self, ConnectionPool, Table};
use depot_core::data_object::{self};
use redis::{self, Commands, PipelineCommands};

use error::{Error, Result};

pub struct OriginKeysTable {
    pool: Arc<ConnectionPool>,
}

impl OriginKeysTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        OriginKeysTable { pool: pool }
    }

    pub fn all(&self, origin: &str) -> Result<Vec<data_object::OriginKeyIdent>> {
        let conn = self.pool().get().unwrap();
        match conn.smembers::<String, Vec<String>>(Self::key(&origin.to_string())) {
            Ok(ids) => {
                let ids = ids.iter()
                             .map(|rev| {
                                 data_object::OriginKeyIdent::new(origin.to_string(),
                                                     rev.clone(),
                                                    format!("/origins/{}/keys/{}",
                                                            &origin, &rev))
                             })
                             .collect();
                Ok(ids)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn write(&self, origin: &str, revision: &str) -> Result<()> {
        let conn = self.pool().get().unwrap();
        try!(conn.sadd(OriginKeysTable::key(&origin.to_string()), revision));
        Ok(())
    }

    pub fn exists(&self, origin: &str, revision: &str) -> Result<bool> {
        let conn = self.pool().get().unwrap();
        let val = try!(conn.sismember(OriginKeysTable::key(&origin.to_string()), revision));
        Ok(val)
    }

    /// return the latest revision for a given origin key
    pub fn latest(&self, origin: &str) -> Result<String> {
        let conn = self.pool().get().unwrap();
        let key = OriginKeysTable::key(&origin.to_string());

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
                Ok(ids[0].to_string())
            }
            Err(e) => Err(Error::from(e)),
        }
    }

}

impl Table for OriginKeysTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin_keys"
    }
}

/// Origin secret keys table
pub struct OriginSecretKeysTable {
    pool: Arc<ConnectionPool>,
}

impl OriginSecretKeysTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        OriginSecretKeysTable { pool: pool }
    }

    // TODO: is storing the keys as plain text ok?
    // TODO: do we use user/service keys to encrypt/decrypt in-memory keys?
    pub fn write(&self, origin: &str, _revision: &str, body: &str) -> Result<()> {
        let conn = self.pool().get().unwrap();
        try!(conn.set(OriginSecretKeysTable::key(&origin.to_string()), body));
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<String> {
        let conn = self.pool().get().unwrap();
        match conn.get::<String, String>(Self::key(&id.to_string())) {
            Ok(body) => {
                Ok(body)
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl Table for OriginSecretKeysTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin_secret_keys"
    }
}

