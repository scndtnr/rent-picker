use opentelemetry::sdk::trace::Tracer;
use tracing::Subscriber;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{
    filter::{Filtered, Targets},
    registry::LookupSpan,
    Layer,
};

/// app log

/// (app log) OpenTelemetry の Trace 情報を Collector に送信するレイヤー
pub(crate) fn otel_trace_layer_of_app<S>(
    service_name: &str,
) -> Filtered<OpenTelemetryLayer<S, Tracer>, Targets, S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let filter = crate::filter::app_only(false);

    crate::layer::open_telemetry::otel_trace_layer(service_name).with_filter(filter)
}

/// (app log) OpenTelemetry の Metrics 情報を Collector に送信するレイヤー
pub(crate) fn otel_metrics_layer_of_app<S>() -> Filtered<MetricsLayer, Targets, S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let filter = crate::filter::app_only(false);

    crate::layer::open_telemetry::otel_metrics_layer().with_filter(filter)
}
