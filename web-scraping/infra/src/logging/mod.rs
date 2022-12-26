mod filter;
mod filtered_layer;
mod layer;
mod writer;

use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use usecase::env::get_env_var;

enum LogType {
    App,
    Db,
    #[allow(unused)]
    System,
}

/// log (tracing) の初期化をする関数
/// アプリケーションの処理実行前に利用する
pub fn init_logging() {
    let service_name = get_env_var("SERVICE_NAME").unwrap();
    // ログ設定にフィルタ・フォーマットを登録し適用する
    tracing_subscriber::registry()
        // --- OpenTelemetry layer ---
        .with(filtered_layer::otel_trace_layer_of_app(&service_name))
        .with(filtered_layer::otel_metrics_layer_of_app())
        // .with(filtered_layer::otel_trace_layer_not_filtered())
        // .with(filtered_layer::otel_metrics_layer_not_filtered())
        // --- bunyan formatting layer ---
        .with(JsonStorageLayer)
        .with(filtered_layer::bunyan_stdio_of_app(&service_name))
        // .with(filtered_layer::bunyan_stdio_of_db(&service_name))
        // .with(filtered_layer::bunyan_stdio_filtered_by_level(&service_name))
        .with(filtered_layer::bunyan_file_of_app(&service_name))
        .with(filtered_layer::bunyan_file_of_db(&service_name))
        // .with(filtered_layer::bunyan_file_not_filtered(&service_name))
        .init();
}

/// log (opentelemetry) を終了する関数
/// アプリケーションの処理終了後に利用する
pub async fn shutdown_logging() {
    // 5秒間隔でOpenTelemetry Collector にSignalsを送信しているので、
    // 終了するまえに5秒待っておく。
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    opentelemetry::global::shutdown_tracer_provider();
}
