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
    pub fn writer_pool(&self) -> Arc<Pool<Sqlite>> {
        Arc::clone(&self.pool.writer_pool)
    }
    pub fn reader_pool(&self) -> Arc<Pool<Sqlite>> {
        Arc::clone(&self.pool.reader_pool)
    }
}
