use tracing::{metadata::LevelFilter, Subscriber};
use tracing_appender::rolling::RollingFileAppender;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{
    filter::{Filtered, Targets},
    registry::LookupSpan,
    Layer,
};

use crate::logging::{filter, layer, writer, LogType};

// type: filtered by targets

type BunyanStdioLayerFilterdByTargets<S> =
    Filtered<BunyanFormattingLayer<fn() -> std::io::Stdout>, Targets, S>;
type BunyanRollingFileLayerFilterdByTargets<S> =
    Filtered<BunyanFormattingLayer<RollingFileAppender>, Targets, S>;

// type: filtered by level filter

#[allow(unused)]
type BunyanStdioLayerFilterdByLevel<S> =
    Filtered<BunyanFormattingLayer<fn() -> std::io::Stdout>, LevelFilter, S>;
type BunyanRollingFileLayerFilterdByLevel<S> =
    Filtered<BunyanFormattingLayer<RollingFileAppender>, LevelFilter, S>;

// app log

/// (app log) bunyan形式で標準出力に書き込むフォーマッタ
pub(crate) fn bunyan_stdio_of_app<S>(name: &str) -> BunyanStdioLayerFilterdByTargets<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let stdio_filter = filter::app_only(true);

    layer::bunyan::bunyan_stdio_format(name).with_filter(stdio_filter)
}

/// (app log) bunyan形式でファイルに書き込むフォーマッタ
pub(crate) fn bunyan_file_of_app<S>(name: &str) -> BunyanRollingFileLayerFilterdByTargets<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let file_filter = filter::app_only(false);
    let make_writer = writer::rolling_file(LogType::App);

    layer::bunyan::bunyan_file_format(name, make_writer).with_filter(file_filter)
}

// db log

/// (db log) bunyan形式で標準出力に書き込むフォーマッタ
#[allow(unused)]
pub(crate) fn bunyan_stdio_of_db<S>(name: &str) -> BunyanStdioLayerFilterdByTargets<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let stdio_filter = filter::db_only(true);

    layer::bunyan::bunyan_stdio_format(name).with_filter(stdio_filter)
}

/// (db log) bunyan形式でファイルに書き込むフォーマッタ
pub(crate) fn bunyan_file_of_db<S>(name: &str) -> BunyanRollingFileLayerFilterdByTargets<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let file_filter = filter::db_only(false);
    let make_writer = writer::rolling_file(LogType::Db);

    layer::bunyan::bunyan_file_format(name, make_writer).with_filter(file_filter)
}

// system log

/// (system log) bunyan形式で標準出力に書き込むフォーマッタ
#[allow(unused)]
pub(crate) fn bunyan_stdio_filtered_by_level<S>(name: &str) -> BunyanStdioLayerFilterdByLevel<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let stdio_filter = filter::system(true);

    layer::bunyan::bunyan_stdio_format(name).with_filter(stdio_filter)
}

/// (system log) bunyan形式でファイルに書き込むフォーマッタ
#[allow(unused)]
pub(crate) fn bunyan_file_not_filtered<S>(name: &str) -> BunyanRollingFileLayerFilterdByLevel<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let file_filter = filter::system(false);
    let make_writer = writer::rolling_file(LogType::System);

    layer::bunyan::bunyan_file_format(name, make_writer).with_filter(file_filter)
}
