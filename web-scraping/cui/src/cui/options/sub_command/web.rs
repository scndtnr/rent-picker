use crate::cui::options::{Area, Service, Table};

#[derive(Debug, Clone, clap::Args, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Web {
    #[arg(short, long, value_enum, default_value_t=Service::Suumo, help = "対象サービスを指定する")]
    pub(crate) service: Service,
    #[arg(short, long, value_enum, default_value_t=Table::RoomHeader, help = "取得したい情報を指定する")]
    pub(crate) table: Table,
    #[arg(short, long, value_enum, default_value_t=Area::Tokyo, help = "検索対象エリアを指定する")]
    pub(crate) area: Area,
    #[arg(help = "最寄り駅を指定する")]
    pub(crate) station: String,
    #[arg(long, help = "取得データをデータベースに保存するboolフラグ")]
    pub(crate) save: bool,
    #[arg(long, help = "ヘッダ情報はデータベースから読む出すboolフラグ")]
    pub(crate) headers_from_database: bool,
    #[arg(long, help = "各ページのスクレイピングはスキップするboolフラグ")]
    pub(crate) dry_run: bool,
}
