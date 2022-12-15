use derive_new::new;
use domain::model::TableType;

#[derive(Debug, Clone, Default)]
pub struct Sql {
    pub room_header: RoomHeaderSql,
}

impl Sql {
    pub fn new() -> Self {
        Self {
            room_header: RoomHeaderSql::new(),
        }
    }
}

#[derive(Debug, Clone, new, Default)]
pub struct RoomHeaderSql;

impl RoomHeaderSql {
    /// テーブル名を決定する
    pub fn table_name<'a>(&self, table: &'a TableType) -> &'a str {
        match table {
            TableType::Main => "room_header",
            TableType::Load => "load_room_header",
            TableType::Temp => "temp_room_header",
        }
    }

    /// PK毎に作成日時が最大のレコードを集約する select 文
    pub fn select_group_by_pk(&self, table: TableType) -> String {
        let table = self.table_name(&table);
        format!(
            "
        SELECT
            t.url
            ,t.residence_title
            ,t.residence_address
            ,t.residence_nearest_station
            ,t.residence_age
            ,t.residence_floors
            ,t.residence_transfer
            ,t.residence_area
            ,t.residence_station
            ,t.room_floor
            ,t.room_rent_price
            ,t.room_condo_fee
            ,t.room_deposit
            ,t.room_key_money
            ,t.room_layout
            ,t.room_exclusive_area
            ,t.created_at
        FROM
            {} t
            JOIN (
                SELECT
                    url,
                    max(created_at) max_created_at
                FROM
                    {}
                GROUP BY
                    url
            ) g
                ON t.url = g.url
                AND t.created_at = g.max_created_at
        ",
            table, table
        )
    }

    /// room_header 系テーブルへの insert文
    pub fn insert_all_column(&self, table: TableType) -> String {
        let table = self.table_name(&table);
        format!(
            "
            INSERT INTO {}
                (
                    url
                    ,residence_title
                    ,residence_address
                    ,residence_nearest_station
                    ,residence_age
                    ,residence_floors
                    ,residence_transfer
                    ,residence_area
                    ,residence_station
                    ,room_floor
                    ,room_rent_price
                    ,room_condo_fee
                    ,room_deposit
                    ,room_key_money
                    ,room_layout
                    ,room_exclusive_area
                    ,created_at
                )
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ",
            table
        )
    }

    /// room_header 系テーブルからテーブルへの insert文
    pub fn insert_all_from_table_to_table(&self, from: TableType, to: TableType) -> String {
        let from = self.table_name(&from);
        let to = self.table_name(&to);
        format!(
            "
                INSERT INTO {}
                SELECT * FROM {}
            ",
            to, from
        )
    }

    /// room_header 系テーブルのデータを全削除する delete文
    pub fn delete_all(&self, table: TableType) -> String {
        let table = self.table_name(&table);
        format!(
            "
            DELETE FROM {}
            ",
            table
        )
    }

    /// room_header 系テーブルからPKに合致したレコードを削除する delete文
    pub fn delete_by_pk(&self, table: TableType) -> String {
        let table = self.table_name(&table);
        format!(
            "
            DELETE FROM {}
            WHERE
                url = ?
            ",
            table
        )
    }

    /// room_header 系テーブルからPKに合致したレコードを削除する delete文
    /// PKは他のテーブルから引っ張ってくる
    pub fn delete_where_group_by_pk_from_other_table(
        &self,
        table: TableType,
        other: TableType,
    ) -> String {
        let table = self.table_name(&table);
        let group_by_pk_from_other = self.select_group_by_pk(other);
        format!(
            "
                DELETE FROM {}
                WHERE
                    url in (
                        SELECT
                            other.url
                        FROM
                            ({}) other
                    )
                ",
            table, group_by_pk_from_other
        )
    }
}
