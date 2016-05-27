// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::sync::Arc;

use dbcache::{ConnectionPool, Table};
use r2d2_redis::RedisConnectionManager;
use redis::{self, Commands, PipelineCommands};

use error::Result;

use tables::*;

pub struct DataStore {
    pub pool: Arc<ConnectionPool>,
    pub packages: PackagesTable,
    pub views: ViewsTable,
    pub origin_keys: OriginKeysTable,
    pub origin_secret_keys: OriginSecretKeysTable,
}

impl DataStore {
    pub fn open<C: redis::IntoConnectionInfo>(config: C) -> Result<Self> {
        // JW TODO: tune pool from config?
        let pool_cfg = Default::default();
        let manager = RedisConnectionManager::new(config).unwrap();
        let pool = Arc::new(ConnectionPool::new(pool_cfg, manager).unwrap());

        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let pool3 = pool.clone();
        let pool4 = pool.clone();

        let packages = PackagesTable::new(pool1);
        let views = ViewsTable::new(pool2);
        let origin_keys = OriginKeysTable::new(pool3);
        let origin_secret_keys = OriginSecretKeysTable::new(pool4);

        Ok(DataStore {
            pool: pool,
            packages: packages,
            views: views,
            origin_keys: origin_keys,
            origin_secret_keys: origin_secret_keys,
        })
    }

    /// Truncates every database in the datastore.
    ///
    /// # Failures
    ///
    /// * If a read-write transaction could not be acquired for any of the databases in the
    ///   datastore
    pub fn clear(&self) -> Result<()> {
        try!(redis::cmd("FLUSHDB").query(self.pool.get().unwrap().deref()));
        Ok(())
    }

    pub fn key_count(&self) -> Result<usize> {
        let count = try!(redis::cmd("DBSIZE").query(self.pool.get().unwrap().deref()));
        Ok(count)
    }
}


