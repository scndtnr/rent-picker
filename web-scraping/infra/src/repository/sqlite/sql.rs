use derive_new::new;

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
    /// PK毎に作成日時が最大のレコードを集約する select 文
    pub fn group_by_pk_from_load_table(&self) -> String {
        "
        WITH group_by_url AS (
            SELECT
                url,
                max(created_at) max_created_at
            FROM
                load_room_header
            GROUP BY
                url
        )
        SELECT
            lrh.url,
            lrh.residence_title,
            lrh.residence_transfer,
            lrh.residence_area,
            lrh.residence_station,
            lrh.created_at
        FROM
            load_room_header lrh
            JOIN group_by_url g
                ON lrh.url = g.url
                AND lrh.created_at = g.max_created_at
        "
        .to_string()
    }

    /// room_header 系テーブルへの insert文
    pub fn insert_all_column(&self, table: &str) -> String {
        format!(
            "
            INSERT INTO {}
                (
                    url
                    ,residence_title
                    ,residence_transfer
                    ,residence_area
                    ,residence_station
                    ,created_at
                )
            VALUES
                (?, ?, ?, ?, ?, ?)
        ",
            table
        )
    }
}
