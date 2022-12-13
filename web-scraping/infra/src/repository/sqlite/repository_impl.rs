use crate::persistence::sqlite::SqliteDb;
use derive_new::new;
use sqlx::{Pool, Sqlite};
use std::{marker::PhantomData, sync::Arc};

#[derive(new, Debug, Clone)]
pub struct SqliteRepositoryImpl<T> {
    pool: SqliteDb,
    _marker: PhantomData<fn() -> T>,
}

impl<T> SqliteRepositoryImpl<T> {
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool.0
    }
    pub fn pool_clone(&self) -> Arc<Pool<Sqlite>> {
        Arc::clone(&self.pool.0)
    }
}
