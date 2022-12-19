use std::{fmt::Write, sync::Arc};

use indicatif::{FormattedDuration, HumanDuration, ProgressBar, ProgressState, ProgressStyle};

async fn progress_bar_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>9}/{len:9}  ({eta}) {msg}",
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
        write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
    })
    .progress_chars("#>-")
}

pub(crate) async fn new_progress_bar(total: u64) -> Arc<ProgressBar> {
    let style = progress_bar_style().await;
    let pb = ProgressBar::new(total);
    pb.set_style(style);
    Arc::new(pb)
}

pub(crate) async fn debug_progress(pb: &ProgressBar, msg: &str) {
    tracing::debug!(
        "[{}] {:>9}/{:<9}  ({:#}) {}",
        FormattedDuration(pb.elapsed()),
        pb.position(),
        pb.length().unwrap_or(0),
        HumanDuration(pb.eta()),
        msg
    );
}
