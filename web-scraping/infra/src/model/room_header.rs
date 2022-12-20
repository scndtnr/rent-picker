use domain::model::{Jst, RoomHeader};
use sqlx::{types::chrono::NaiveDateTime, FromRow};

#[derive(FromRow, Debug)]
pub struct RoomHeaderRecord {
    pub url: String,
    pub building_name: String,
    pub location: String,
    pub walk_to_station: String,
    pub age_in_years: String,
    pub number_of_floors: String,
    pub transfer_in_search_result: String,
    pub area_of_search_condition: String,
    pub commute_station_of_search_condition: String,
    pub floor: String,
    pub rental_fee: String,
    pub management_fee: String,
    pub security_deposit: String,
    pub key_money: String,
    pub floor_plan: String,
    pub private_area: String,
    pub scraping_date: NaiveDateTime,
}

impl From<RoomHeader> for RoomHeaderRecord {
    fn from(header: RoomHeader) -> Self {
        Self {
            url: header.url().to_string(),
            building_name: header.building_name().to_string(),
            location: header.location().to_string(),
            walk_to_station: header.walk_to_station().to_string(),
            age_in_years: header.age_in_years().to_string(),
            number_of_floors: header.number_of_floors().to_string(),
            transfer_in_search_result: header.transfer_in_search_result().to_string(),
            area_of_search_condition: header.area_of_search_condition().to_string(),
            commute_station_of_search_condition: header
                .commute_station_of_search_condition()
                .to_string(),
            floor: header.floor().to_string(),
            rental_fee: header.rental_fee().to_string(),
            management_fee: header.management_fee().to_string(),
            security_deposit: header.security_deposit().to_string(),
            key_money: header.key_money().to_string(),
            floor_plan: header.floor_plan().to_string(),
            private_area: header.private_area().to_string(),
            scraping_date: header.scraping_date().naive_utc(),
        }
    }
}

impl TryFrom<RoomHeaderRecord> for RoomHeader {
    type Error = anyhow::Error;
    fn try_from(record: RoomHeaderRecord) -> Result<Self, Self::Error> {
        Ok(Self::new(
            record.url,
            record.building_name,
            record.location,
            record.walk_to_station,
            record.age_in_years,
            record.number_of_floors,
            record.transfer_in_search_result,
            record.area_of_search_condition.try_into()?,
            record.commute_station_of_search_condition,
            record.floor,
            record.rental_fee,
            record.management_fee,
            record.security_deposit,
            record.key_money,
            record.floor_plan,
            record.private_area,
            Jst::from_utc_datetime(&record.scraping_date),
        ))
    }
}
