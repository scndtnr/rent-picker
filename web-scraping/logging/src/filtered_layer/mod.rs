#[cfg(feature = "bunyan")]
mod bunyan;

#[cfg(feature = "bunyan")]
pub(super) use bunyan::{bunyan_file_of_app, bunyan_file_of_db, bunyan_stdio_of_app};

#[cfg(feature = "bunyan")]
#[allow(unused_imports)]
pub(super) use bunyan::{
    bunyan_file_not_filtered, bunyan_stdio_filtered_by_level, bunyan_stdio_of_db,
};

#[cfg(feature = "otel")]
mod open_telemetry;

#[cfg(feature = "otel")]
pub(super) use open_telemetry::{otel_metrics_layer_of_app, otel_trace_layer_of_app};

#[cfg(feature = "otel")]
#[allow(unused_imports)]
pub(super) use open_telemetry::{otel_metrics_layer_not_filtered, otel_trace_layer_not_filtered};
