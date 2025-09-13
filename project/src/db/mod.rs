//! Postgres connection pool setup using deadpool-postgres

use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::env;
use std::str::FromStr;

pub type PgPool = Pool;

/// Create a new Postgres connection pool using DATABASE_URL from environment.
///
/// Panics if DATABASE_URL is not set or invalid.
pub fn create_pool() -> PgPool {
    // Load environment variables from .env if present
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let cfg = Config::from_str(&db_url).expect("Invalid DATABASE_URL");
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(cfg, NoTls, mgr_config);
    Pool::builder(mgr)
        .max_size(16)
        .build()
        .expect("Failed to build Postgres pool")
}
