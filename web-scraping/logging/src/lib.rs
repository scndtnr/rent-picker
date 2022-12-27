mod env;
mod filter;
mod filtered_layer;
mod init;
mod layer;
mod writer;

enum LogType {
    App,
    Db,
    #[allow(unused)]
    System,
}

/// log (tracing) の初期化をする関数
/// アプリケーションの処理実行前に利用する
pub fn init_logging() {
    cfg_if::cfg_if! {
        if #[cfg(all(feature = "bunyan", feature = "stackdriver"))] {
            compile_error!("Error: Feature \"bunyan\" and feature \"stackdriver\" cannot be enabled at the same time.");
        } else if #[cfg(all(feature = "bunyan", feature = "otel"))] {
            init::init_logging_with_bunyan_and_otel();
        } else if #[cfg(feature = "bunyan")] {
            init::init_logging_with_bunyan();
        } else if #[cfg(feature = "stackdriver")] {
            init::init_logging_with_stackdriver();
        } else {
            compile_error!("Error: Feature must be specified as either 'bunyan' or 'stackdriver'. Please select one of these options and try again.")
        }
    };
}

/// log (opentelemetry) を終了する関数
/// アプリケーションの処理終了後に利用する
#[cfg(feature = "otel")]
pub async fn shutdown_logging() {
    opentelemetry::global::shutdown_tracer_provider();
}
