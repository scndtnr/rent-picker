use crate::cui::options::{DataAction, Table, TableType};

#[derive(Debug, Clone, clap::Args, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Db {
    #[arg(long, value_enum, default_value_t=DataAction::Summary, help = "データに対する処理を指定する")]
    pub(crate) action: DataAction,
    #[arg(long, value_enum, default_value_t=Table::RoomHeader, help = "selectしたいテーブルを指定する")]
    pub(crate) table: Table,
    #[arg(long, value_enum, default_value_t=TableType::Main, help = "selectしたいテーブルの種類を指定する")]
    pub(crate) table_type: TableType,
}
