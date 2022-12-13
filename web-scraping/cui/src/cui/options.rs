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
}

#[derive(Debug, Clone, clap::Subcommand, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Task {
    /// ターゲットのヘルスチェックをする
    HealthCheck(HealthCheck),
    /// Webスクレイピングをする
    WebScrape(WebScrape),
}

#[derive(Debug, Clone, clap::Args, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct HealthCheck {
    #[arg(short, long, value_enum, default_value_t=Service::Suumo, help = "対象サービスを指定する")]
    pub(super) target: Service,
}

#[derive(Debug, Clone, clap::Args, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct WebScrape {
    #[arg(short, long, value_enum, default_value_t=Service::Suumo, help = "対象サービスを指定する")]
    pub(super) service: Service,
    #[arg(short, long, value_enum, default_value_t=Item::RoomHeaders, help = "取得したい情報を指定する")]
    pub(super) item: Item,
    #[arg(short, long, value_enum, default_value_t=Area::Tokyo, help = "検索対象エリアを指定する")]
    pub(super) area: Area,
    #[arg(help = "最寄り駅を指定する")]
    pub(super) station: String,
    #[arg(long, help = "取得データをデータベースに保存するboolフラグ")]
    pub(super) save: bool,
    #[arg(long, help = "ヘッダ情報はデータベースから読む出すboolフラグ")]
    pub(super) headers_from_database: bool,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Service {
    Suumo,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Item {
    Rooms,
    RoomHeaders,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Area {
    Tokyo,
    Kanagawa,
    Saitama,
    Chiba,
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Area::Tokyo => write!(f, "Tokyo"),
            Area::Kanagawa => write!(f, "Kanagawa"),
            Area::Saitama => write!(f, "Saitama"),
            Area::Chiba => write!(f, "Chiba"),
        }
    }
}
