use domain::model::{Jst, RawRoom};
use sqlx::{types::chrono::NaiveDateTime, FromRow};

#[derive(FromRow, Debug)]
pub struct RawRoomRecord {
    pub url: String,
    pub redirect_url: String,
    pub suumo_code: Option<String>,
    pub building_name: Option<String>,
    pub rental_fee: Option<String>,
    pub management_fee: Option<String>,
    pub security_deposit: Option<String>,
    pub key_money: Option<String>,
    pub guarantee_deposit: Option<String>,
    pub key_money_amortization: Option<String>,
    pub location: Option<String>,
    pub walk_to_station: Option<String>,
    pub floor_plan: Option<String>,
    pub floor_plan_details: Option<String>,
    pub private_area: Option<String>,
    pub age_in_years: Option<String>,
    pub construction_date_yyyymm: Option<String>,
    pub floor: Option<String>,
    pub number_of_floors: Option<String>,
    pub facing_direction: Option<String>,
    pub building_type: Option<String>,
    pub features: Option<String>,
    pub structure: Option<String>,
    pub damage_insurance: Option<String>,
    pub parking: Option<String>,
    pub move_in: Option<String>,
    pub transaction_type: Option<String>,
    pub conditions: Option<String>,
    pub property_code: Option<String>,
    pub contract_period: Option<String>,
    pub notes: Option<String>,
    pub info_update_date: Option<NaiveDateTime>,
    pub next_update_date: Option<NaiveDateTime>,
    pub scraping_date: NaiveDateTime,
    pub is_expired: usize,
}

impl From<RawRoom> for RawRoomRecord {
    fn from(room: RawRoom) -> Self {
        Self {
            url: room.url().to_string(),
            redirect_url: room.redirect_url().to_string(),
            suumo_code: room.suumo_code().clone(),
            building_name: room.building_name().clone(),
            rental_fee: room.rental_fee().clone(),
            management_fee: room.management_fee().clone(),
            security_deposit: room.security_deposit().clone(),
            key_money: room.key_money().clone(),
            guarantee_deposit: room.guarantee_deposit().clone(),
            key_money_amortization: room.key_money_amortization().clone(),
            location: room.location().clone(),
            walk_to_station: room.walk_to_station().clone(),
            floor_plan: room.floor_plan().clone(),
            floor_plan_details: room.floor_plan_details().clone(),
            private_area: room.private_area().clone(),
            age_in_years: room.age_in_years().clone(),
            construction_date_yyyymm: room.construction_date_yyyymm().clone(),
            floor: room.floor().clone(),
            number_of_floors: room.number_of_floors().clone(),
            facing_direction: room.facing_direction().clone(),
            building_type: room.building_type().clone(),
            features: room.features().clone(),
            structure: room.structure().clone(),
            damage_insurance: room.damage_insurance().clone(),
            parking: room.parking().clone(),
            move_in: room.move_in().clone(),
            transaction_type: room.transaction_type().clone(),
            conditions: room.conditions().clone(),
            property_code: room.property_code().clone(),
            contract_period: room.contract_period().clone(),
            notes: room.notes().clone(),
            info_update_date: room.info_update_date().map(|dt| dt.naive_utc()),
            next_update_date: room.next_update_date().map(|dt| dt.naive_utc()),
            scraping_date: room.scraping_date().naive_utc(),
            is_expired: room.is_expired() as usize,
        }
    }
}

impl TryFrom<RawRoomRecord> for RawRoom {
    type Error = anyhow::Error;
    fn try_from(record: RawRoomRecord) -> Result<Self, Self::Error> {
        Ok(Self::new(
            record.url,
            record.redirect_url,
            record.suumo_code,
            record.building_name,
            record.rental_fee,
            record.management_fee,
            record.security_deposit,
            record.key_money,
            record.guarantee_deposit,
            record.key_money_amortization,
            record.location,
            record.walk_to_station,
            record.floor_plan,
            record.floor_plan_details,
            record.private_area,
            record.age_in_years,
            record.construction_date_yyyymm,
            record.floor,
            record.number_of_floors,
            record.facing_direction,
            record.building_type,
            record.features,
            record.structure,
            record.damage_insurance,
            record.parking,
            record.move_in,
            record.transaction_type,
            record.conditions,
            record.property_code,
            record.contract_period,
            record.notes,
            record
                .info_update_date
                .map(|dt| Jst::from_utc_datetime(&dt)),
            record
                .next_update_date
                .map(|dt| Jst::from_utc_datetime(&dt)),
            Jst::from_utc_datetime(&record.scraping_date),
            match record.is_expired {
                1 => true,
                0 => false,
                _ => unreachable!("is_expired must be bool ( 0 or 1 )"),
            },
        ))
    }
}
