use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use std::sync::Arc;
use usecase::env::get_env_var;

#[derive(Clone, Debug)]
pub struct SqliteDb {
    pub(crate) writer_pool: Arc<Pool<Sqlite>>,
    pub(crate) reader_pool: Arc<Pool<Sqlite>>,
}

impl SqliteDb {
    pub async fn new() -> Self {
        let filename = get_env_var("SQLITE_FILE_PATH")
            .expect("For sqlite connection, 'SQLITE_FILE_PATH' must be set!");
        let max_concurrency = get_env_var("MAX_CONCURRENCY")
            .expect("To specify max connections, 'MAX_CONCURRENCY' must be set!")
            .parse()
            .expect("MAX_CONCURRENCY must be unsigned integer!");
        let writer = Self::writer_pool(&filename).await;
        let reader = Self::reader_pool(&filename, max_concurrency).await;

        SqliteDb {
            writer_pool: Arc::new(writer),
            reader_pool: Arc::new(reader),
        }
    }

    async fn writer_pool(filename: &str) -> Pool<Sqlite> {
        let options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(filename);
        SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .expect(
                "Fail to get writer pool to the sqlite database. Please check your configuration.",
            )
    }

    async fn reader_pool(filename: &str, max_concurrency: u32) -> Pool<Sqlite> {
        let options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .read_only(true)
            .filename(filename);
        SqlitePoolOptions::new()
            .max_connections(max_concurrency)
            .connect_with(options)
            .await
            .expect(
                "Fail to get reader pool to the sqlite database. Please check your configuration.",
            )
    }
}
