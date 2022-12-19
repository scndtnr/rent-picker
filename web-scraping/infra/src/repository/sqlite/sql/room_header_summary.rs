use domain::model::TableType;

/// PK毎に作成日時が最大のレコードを集約する select 文
pub fn group_by_area_and_station(table: &TableType) -> String {
    let table = super::room_header::table_name(table);
    format!(
        "
        SELECT 
            residence_area ,
            residence_station, 
            count(*) as record_count,
            round((count(*) * 1.2) /3600, 2) as estimated_hour_for_scraping 
        FROM 
            {}
        GROUP BY
            residence_area ,
            residence_station
        ",
        table
    )
}
