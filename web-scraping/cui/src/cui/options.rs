/// コマンドライン引数のパース用構造体
#[derive(Debug, Clone, clap::Parser, PartialEq, Eq, PartialOrd, Ord)]
#[clap(
    name = "rent-picker",
    version = "0.1.0",
    author = "zumi",
    about = "Tool to get rental information by web scraping in Rust"
)]
pub(crate) struct Options {
    #[command(subcommand, help = "実施したい処理を指定する")]
    pub(super) task: Task,
    #[arg(short, long, value_enum, default_value_t=Target::Suumo, help = "対象サービスを指定する")]
    pub(super) target: Target,
    #[arg(long, help = "仮実行にするboolフラグ")]
    pub(super) dry_run: bool,
}

#[derive(Debug, Clone, clap::Subcommand, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Task {
    /// ターゲットのヘルスチェックをする
    HealthCheck,
    /// Webスクレイピングをする
    WebScrape,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Target {
    /// Suumoの賃貸情報を対象とする
    Suumo,
}
