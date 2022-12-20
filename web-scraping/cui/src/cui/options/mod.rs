mod sub_command;
mod value_enum;

pub(crate) use sub_command::{
    db::Db,
    health_check::HealthCheck,
    web::{self, Web},
};
pub(crate) use value_enum::{Area, DataAction, Service, Table, TableType};

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
    Web(Web),
    /// データベースからデータを読み出す
    Db(Db),
}
