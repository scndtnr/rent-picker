use crate::env::get_env_var;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite, SqlitePool,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SqliteDb(pub(crate) Arc<Pool<Sqlite>>);

impl SqliteDb {
    pub async fn new() -> Self {
        let max_concurrency = get_env_var("MAX_CONCURRENCY")
            .expect("To specify max connections, 'MAX_CONCURRENCY' must be set!")
            .parse()
            .expect("MAX_CONCURRENCY must be unsigned integer!");
        let filename = get_env_var("SQLITE_FILE_PATH")
            .expect("For sqlite connection, 'SQLITE_FILE_PATH' must be set!");
        let options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(filename);
        let pool = SqlitePoolOptions::new()
            .max_connections(max_concurrency)
            .connect_with(options)
            .await
            .expect("Fail to connect to the sqlite database. Please check your configuration.");

        SqliteDb(Arc::new(pool))
    }
}
