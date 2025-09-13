//! Postgres connection pool setup using deadpool-postgres

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::env;
use tokio_postgres::{Config, NoTls};

pub type PgPool = Pool;

pub fn create_pool() -> PgPool {
    let mut cfg = Config::new();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    cfg.connect_str(&db_url).unwrap();
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(cfg, NoTls, mgr_config);
    Pool::builder(mgr).max_size(16).build().unwrap()
}
