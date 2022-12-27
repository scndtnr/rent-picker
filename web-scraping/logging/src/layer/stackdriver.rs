use tracing::Subscriber;
use tracing_subscriber::{fmt::MakeWriter, registry::LookupSpan};

/// stackdriver形式で標準出力に書き込むレイヤー
pub(crate) fn stackdriver_stdio_layer<S>() -> tracing_stackdriver::Layer<S>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    tracing_stackdriver::layer()
}

/// stackdriver形式でファイルに書き込むレイヤー
pub(crate) fn stackdriver_file_layer<S, W>(make_writer: W) -> tracing_stackdriver::Layer<S, W>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    W: for<'a> MakeWriter<'a> + 'static,
{
    tracing_stackdriver::layer().with_writer(make_writer)
}
