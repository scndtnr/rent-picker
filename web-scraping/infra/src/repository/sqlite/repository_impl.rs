use crate::persistence::sqlite::SqliteDb;
use derive_new::new;
use sqlx::{Pool, Sqlite};
use std::marker::PhantomData;

#[derive(new, Debug, Clone)]
pub struct SqliteRepositoryImpl<T> {
    pool: SqliteDb,
    _marker: PhantomData<fn() -> T>,
}

impl<T> SqliteRepositoryImpl<T> {
    pub fn writer_pool(&self) -> &Pool<Sqlite> {
        &self.pool.writer_pool
    }
    pub fn reader_pool(&self) -> &Pool<Sqlite> {
        &self.pool.reader_pool
    }
}
