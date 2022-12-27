use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{env::get_env_var, filtered_layer};

/// bunyan と OpenTelemetry を併用する場合の初期化処理
#[cfg(all(feature = "bunyan", feature = "otel"))]
pub(crate) fn init_logging_with_bunyan_and_otel() {
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

/// bunyan のみを使用する場合の初期化処理
#[cfg(feature = "bunyan")]
pub(crate) fn init_logging_with_bunyan() {
    let service_name = get_env_var("SERVICE_NAME").unwrap();
    // ログ設定にフィルタ・フォーマットを登録し適用する
    tracing_subscriber::registry()
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

/// stackdriver のみを使用する場合の初期化処理
#[cfg(feature = "stackdriver")]
pub(crate) fn init_logging_with_stackdriver() {
    todo!()
}
