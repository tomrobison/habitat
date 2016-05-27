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

pub struct OriginsTable {
    pool: Arc<ConnectionPool>,
}

impl OriginsTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        OriginsTable { pool: pool }
    }

    pub fn list_members(&self, origin: &str) -> Result<Vec<String>> {
        let conn = self.pool().get().unwrap();
        match conn.smembers::<String, Vec<String>>(Self::key(&origin.to_string())) {
            Ok(ids) => {
                let ids = ids.iter()
                             .map(|rev| {
                                 rev.clone()
                             })
                             .collect();
                Ok(ids)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn create(&self, origin: &str, owner: &str) -> Result<()> {
        let conn = self.pool().get().unwrap();
        try!(conn.sadd(OriginsTable::key(&origin.to_string()), owner));
        Ok(())
    }


    pub fn add_member(&self, origin: &str, username: &str) -> Result<()> {
        let conn = self.pool().get().unwrap();
        try!(conn.sadd(OriginsTable::key(&origin.to_string()), username));
        Ok(())
    }

    pub fn delete_member(&self, origin: &str, username: &str) -> Result<()> {
        let conn = self.pool().get().unwrap();
        try!(conn.srem(OriginsTable::key(&origin.to_string()), username));
        Ok(())
    }

    pub fn delete(&self, origin: &str) -> Result<()> {
        let conn = self.pool().get().unwrap();
        try!(conn.del(OriginsTable::key(&origin.to_string())));
        Ok(())
    }

    pub fn is_origin(&self, origin: &str) -> Result<bool> {
        let conn = self.pool().get().unwrap();
        let val = try!(conn.exists(OriginsTable::key(&origin.to_string())));
        Ok(val)
    }

    pub fn is_member(&self, origin: &str, username: &str) -> Result<bool> {
        let conn = self.pool().get().unwrap();
        let val = try!(conn.sismember(OriginsTable::key(&origin.to_string()), username));
        Ok(val)
    }
}

impl Table for OriginsTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origins"
    }
}


