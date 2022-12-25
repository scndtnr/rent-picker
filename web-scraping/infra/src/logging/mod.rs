mod filter;
mod filtered_layer;
mod layer;
mod writer;

use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

enum LogType {
    App,
    Db,
    System,
}

/// log (tracing) の初期化をする関数
/// アプリケーションの処理実行前に利用する
pub fn init_logging(name: &str) {
    // ログ設定にフィルタ・フォーマットを登録し適用する
    tracing_subscriber::registry()
        .with(JsonStorageLayer)
        .with(filtered_layer::bunyan_stdio_of_app(name))
        // .with(filtered_layer::bunyan_stdio_of_db(name))
        // .with(filtered_layer::bunyan_stdio_filtered_by_level(name))
        .with(filtered_layer::bunyan_file_of_app(name))
        .with(filtered_layer::bunyan_file_of_db(name))
        .with(filtered_layer::bunyan_file_not_filtered(name))
        .init();
}

/// log (opentelemetry) を終了する関数
/// アプリケーションの処理終了後に利用する
pub async fn shutdown_logging() {
    todo!()
}
