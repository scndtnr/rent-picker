use chrono::{DateTime, FixedOffset};
use derive_new::new;

use super::TargetArea;

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoomHeader {
    /// 詳細ページのURL
    url: String,
    /// 建物名
    building_name: String,
    /// 所在地
    location: String,
    /// 駅徒歩
    walk_to_station: String,
    /// 築年数
    age_in_years: String,
    /// 階建
    number_of_floors: String,
    /// 乗換回数（検索結果）
    transfer_in_search_result: String,
    /// 都道府県エリア（検索条件）
    area_of_search_condition: TargetArea,
    /// 通勤先の最寄り駅（検索条件）
    commute_station_of_search_condition: String,
    /// 階
    floor: String,
    /// 家賃
    rental_fee: String,
    /// 管理費共益費
    management_fee: String,
    /// 敷金
    security_deposit: String,
    /// 礼金
    key_money: String,
    /// 間取り
    floor_plan: String,
    /// 専有面積
    private_area: String,
    /// スクレイピングした日時
    scraping_date: DateTime<FixedOffset>,
}

impl RoomHeader {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn building_name(&self) -> &str {
        &self.building_name
    }
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn walk_to_station(&self) -> &str {
        &self.walk_to_station
    }
    pub fn age_in_years(&self) -> &str {
        &self.age_in_years
    }
    pub fn number_of_floors(&self) -> &str {
        &self.number_of_floors
    }
    pub fn transfer_in_search_result(&self) -> &str {
        &self.transfer_in_search_result
    }
    pub fn area_of_search_condition(&self) -> &TargetArea {
        &self.area_of_search_condition
    }
    pub fn commute_station_of_search_condition(&self) -> &str {
        &self.commute_station_of_search_condition
    }
    pub fn floor(&self) -> &str {
        &self.floor
    }
    pub fn rental_fee(&self) -> &str {
        &self.rental_fee
    }
    pub fn management_fee(&self) -> &str {
        &self.management_fee
    }
    pub fn security_deposit(&self) -> &str {
        &self.security_deposit
    }
    pub fn key_money(&self) -> &str {
        &self.key_money
    }
    pub fn floor_plan(&self) -> &str {
        &self.floor_plan
    }
    pub fn private_area(&self) -> &str {
        &self.private_area
    }
    pub fn scraping_date(&self) -> DateTime<FixedOffset> {
        self.scraping_date
    }
}
