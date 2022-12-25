use opentelemetry::sdk::{metrics::controllers::BasicController, trace::Tracer};
use opentelemetry_otlp::WithExportConfig;
use tracing::Subscriber;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::registry::LookupSpan;

/// OpenTelemetry の Trace 情報を送信するレイヤー
pub(crate) fn otel_trace_layer<S>() -> OpenTelemetryLayer<S, Tracer>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let tracer = build_tracer();
    tracing_opentelemetry::layer().with_tracer(tracer)
}

/// OpenTelemetry の Metrics 情報を送信するレイヤー
pub(crate) fn otel_metrics_layer() -> MetricsLayer {
    let controller = build_metrics_controller();
    tracing_opentelemetry::MetricsLayer::new(controller)
}

// https://github.com/open-telemetry/opentelemetry-rust/blob/d4b9befea04bcc7fc19319a6ebf5b5070131c486/examples/basic-otlp/src/main.rs#L35-L52
fn build_metrics_controller() -> BasicController {
    opentelemetry_otlp::new_pipeline()
        .metrics(
            opentelemetry::sdk::metrics::selectors::simple::histogram(Vec::new()),
            opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector(),
            opentelemetry::runtime::Tokio,
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .build()
        .expect("Failed to build metrics controller")
}

fn build_tracer() -> Tracer {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn)
                .with_id_generator(opentelemetry::sdk::trace::RandomIdGenerator::default())
                .with_resource(opentelemetry::sdk::Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", "sample-app"),
                ])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Not running in tokio runtime")
}
