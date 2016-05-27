// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.


/*
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

    pub fn all(&self, origin: &str) -> Result<Vec<data_object::OriginyIdent>> {
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
        try!(conn.sadd(OriginsTable::key(&origin.to_string()), revision));
        Ok(())
    }
}

impl Table for OriginsTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin"
    }
}
*/
