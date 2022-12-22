use crate::cui::options::{Area, Service};

#[derive(Debug, Clone, clap::Args, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Web {
    #[arg(short, long, value_enum, default_value_t=Service::Suumo, help = "対象サービスを指定する")]
    pub(crate) service: Service,
    #[command(subcommand, help = "取得したいデータを指定する")]
    pub(crate) target_page: TargetPage,
}

#[derive(Debug, Clone, clap::Subcommand, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TargetPage {
    /// 賃貸情報の概要データをスクレイピングする
    RoomHeader(RoomHeader),
    /// 賃貸情報の詳細データをスクレイピングする
    RawRoom(RawRoom),
}

#[derive(Debug, Clone, clap::Args, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RoomHeader {
    #[arg(help = "最寄り駅を指定する")]
    pub(crate) station: String,
    #[arg(short, long, value_enum, default_value_t=Area::Tokyo, help = "検索対象エリアを指定する")]
    pub(crate) area: Area,
    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "スクレイピングする最大ページ数を指定する"
    )]
    pub(crate) max_page: usize,
    #[arg(long, help = "取得データをデータベースに保存するboolフラグ")]
    pub(crate) save: bool,
    #[arg(long, help = "各ページのスクレイピングはスキップするboolフラグ")]
    pub(crate) dry_run: bool,
}

#[derive(Debug, Clone, clap::Args, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RawRoom {
    #[arg(short, long, value_enum, default_value_t=Area::Tokyo, help = "データベースからの取得対象エリアを指定する")]
    pub(crate) area: Area,
    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "スクレイピングする最大ページ数を指定する"
    )]
    pub(crate) max_page: usize,
    #[arg(long, help = "取得データをデータベースに保存するboolフラグ")]
    pub(crate) save: bool,
    #[arg(long, help = "各ページのスクレイピングはスキップするboolフラグ")]
    pub(crate) dry_run: bool,
}
