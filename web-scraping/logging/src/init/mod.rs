use crate::filtered_layer;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "bunyan")]
use crate::env::get_env_var;
#[cfg(feature = "bunyan")]
use tracing_bunyan_formatter::JsonStorageLayer;

/// bunyan と OpenTelemetry を併用する場合の初期化処理
#[cfg(all(feature = "bunyan", feature = "otel", not(feature = "stackdriver")))]
pub(crate) fn init_logging_with_bunyan_and_otel() {
    let service_name = get_env_var("SERVICE_NAME").unwrap();
    // ログ設定にフィルタ・フォーマットを登録し適用する
    tracing_subscriber::registry()
        // --- OpenTelemetry layer ---
        .with(filtered_layer::otel_trace_layer_of_app(&service_name))
        .with(filtered_layer::otel_metrics_layer_of_app())
        // --- bunyan formatting layer ---
        .with(JsonStorageLayer)
        .with(filtered_layer::bunyan_stdio_of_app(&service_name))
        .with(filtered_layer::bunyan_file_of_app(&service_name))
        .with(filtered_layer::bunyan_file_of_db(&service_name))
        .init();
}

/// bunyan と stackdriver を併用する場合の初期化処理
#[cfg(all(feature = "bunyan", feature = "stackdriver"))]
pub(crate) fn init_logging_with_bunyan_and_stackdriver() {
    let service_name = get_env_var("SERVICE_NAME").unwrap();
    // ログ設定にフィルタ・フォーマットを登録し適用する
    tracing_subscriber::registry()
        // --- Stackdriver layer ---
        .with(filtered_layer::stackdriver_file_of_app())
        .with(filtered_layer::stackdriver_file_of_db())
        // --- bunyan formatting layer ---
        .with(JsonStorageLayer)
        .with(filtered_layer::bunyan_stdio_of_app(&service_name))
        .with(filtered_layer::bunyan_file_of_app(&service_name))
        .with(filtered_layer::bunyan_file_of_db(&service_name))
        .init();
}

/// bunyan のみを使用する場合の初期化処理
#[cfg(all(feature = "bunyan", not(feature = "stackdriver")))]
pub(crate) fn init_logging_with_bunyan() {
    let service_name = get_env_var("SERVICE_NAME").unwrap();
    // ログ設定にフィルタ・フォーマットを登録し適用する
    tracing_subscriber::registry()
        // --- bunyan formatting layer ---
        .with(JsonStorageLayer)
        .with(filtered_layer::bunyan_stdio_of_app(&service_name))
        .with(filtered_layer::bunyan_file_of_app(&service_name))
        .with(filtered_layer::bunyan_file_of_db(&service_name))
        .init();
}

/// stackdriver のみを使用する場合の初期化処理
#[cfg(all(feature = "stackdriver", not(feature = "bunyan")))]
pub(crate) fn init_logging_with_stackdriver() {
    // ログ設定にフィルタ・フォーマットを登録し適用する
    tracing_subscriber::registry()
        // --- stackdriver layer ---
        .with(filtered_layer::stackdriver_stdio_of_app())
        .with(filtered_layer::stackdriver_file_of_app())
        .with(filtered_layer::stackdriver_file_of_db())
        .init();
}
