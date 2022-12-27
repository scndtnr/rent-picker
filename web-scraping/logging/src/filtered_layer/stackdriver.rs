use tracing::Subscriber;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{
    filter::{Filtered, Targets},
    registry::LookupSpan,
    Layer,
};

// app log

/// (app log) stackdriver形式で標準出力に書き込むレイヤー
#[cfg(all(feature = "stackdriver", not(feature = "bunyan")))]
pub(crate) fn stackdriver_stdio_of_app<S>() -> Filtered<tracing_stackdriver::Layer<S>, Targets, S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let filter = crate::filter::app_only(false);
    crate::layer::stackdriver::stackdriver_stdio_layer().with_filter(filter)
}

/// (app log) stackdriver形式でファイルに書き込むレイヤー
pub(crate) fn stackdriver_file_of_app<S>(
) -> Filtered<tracing_stackdriver::Layer<S, RollingFileAppender>, Targets, S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let filter = crate::filter::app_only(false);
    let filename = crate::writer::log_filename(crate::LogType::App);
    let make_writer = crate::writer::rolling_file(format!("stackdriver_{}", filename));
    crate::layer::stackdriver::stackdriver_file_layer(make_writer).with_filter(filter)
}

// db log

/// (db log) stackdriver形式で標準出力に書き込むレイヤー
#[allow(unused)]
pub(crate) fn stackdriver_stdio_of_db<S>() -> Filtered<tracing_stackdriver::Layer<S>, Targets, S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let filter = crate::filter::db_only(false);
    crate::layer::stackdriver::stackdriver_stdio_layer().with_filter(filter)
}

/// (db log) stackdriver形式でファイルに書き込むレイヤー
pub(crate) fn stackdriver_file_of_db<S>(
) -> Filtered<tracing_stackdriver::Layer<S, RollingFileAppender>, Targets, S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let filter = crate::filter::db_only(false);
    let filename = crate::writer::log_filename(crate::LogType::Db);
    let make_writer = crate::writer::rolling_file(format!("stackdriver_{}", filename));
    crate::layer::stackdriver::stackdriver_file_layer(make_writer).with_filter(filter)
}
