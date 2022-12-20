use sqlx::FromRow;

#[derive(Debug)]
pub struct RoomHeaderSummaryTable(pub Vec<RoomHeaderSummaryRecord>);

impl RoomHeaderSummaryTable {
    pub fn total_count(&self) -> u32 {
        self.0.iter().map(|record| record.record_count).sum()
    }
}

impl From<Vec<RoomHeaderSummaryRecord>> for RoomHeaderSummaryTable {
    fn from(records: Vec<RoomHeaderSummaryRecord>) -> Self {
        Self(records)
    }
}

#[derive(FromRow, Debug)]
pub struct RoomHeaderSummaryRecord {
    pub area_of_search_condition: String,
    pub commute_station_of_search_condition: String,
    pub record_count: u32,
    pub estimated_hour_for_scraping: f32,
}
