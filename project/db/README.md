# project/db

This module provides the Postgres connection pool setup using `deadpool-postgres`.

- Edit `mod.rs` to configure the pool.
- Use `create_pool()` to initialize and share the pool in your Actix app.
- Set `DATABASE_URL` in your environment.
