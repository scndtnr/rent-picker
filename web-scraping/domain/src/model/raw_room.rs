use chrono::{DateTime, FixedOffset};
use derive_new::new;

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawRoom {
    /// 詳細ページのURL
    url: String,
    /// suumo物件コード
    suumo_code: String,
    /// 建物名
    building_name: String,
    /// 家賃
    rental_fee: String,
    /// 管理費共益費
    management_fee: String,
    /// 敷金
    security_deposit: String,
    /// 礼金
    key_money: String,
    /// 保証金
    guarantee_deposit: String,
    /// 敷引償却
    key_money_amortization: String,
    /// 所在地
    location: String,
    /// 駅徒歩
    walk_to_station: String,
    /// 間取り
    floor_plan: String,
    /// 間取り詳細
    floor_plan_details: String,
    /// 専有面積
    private_area: String,
    /// 築年数
    age_in_years: String,
    /// 築年月
    construction_date_yyyymm: String,
    /// 階
    floor: String,
    /// 階建
    number_of_floors: String,
    /// 向き
    facing_direction: String,
    /// 建物種別
    building_type: String,
    /// 部屋の特徴設備
    features: String,
    /// 構造
    structure: String,
    /// 損保
    damage_insurance: String,
    /// 駐車場
    parking: String,
    /// 入居（時期）
    move_in: String,
    /// 取引態様
    transaction_type: String,
    /// 条件
    conditions: String,
    /// 取り扱い店舗物件コード
    property_code: String,
    /// 契約期間
    contract_period: String,
    /// 備考
    notes: String,
    /// 情報更新日
    info_update_date: DateTime<FixedOffset>,
    /// 次回更新日
    next_update_date: DateTime<FixedOffset>,
    /// スクレイピングした日時
    scraping_date: DateTime<FixedOffset>,
    /// 掲載終了フラグ
    is_expired: bool,
}

impl RawRoom {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn suumo_code(&self) -> &str {
        &self.suumo_code
    }
    pub fn building_name(&self) -> &str {
        &self.building_name
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
    pub fn guarantee_deposit(&self) -> &str {
        &self.guarantee_deposit
    }
    pub fn key_money_amortization(&self) -> &str {
        &self.key_money_amortization
    }
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn walk_to_station(&self) -> &str {
        &self.walk_to_station
    }
    pub fn floor_plan(&self) -> &str {
        &self.floor_plan
    }
    pub fn floor_plan_details(&self) -> &str {
        &self.floor_plan_details
    }
    pub fn private_area(&self) -> &str {
        &self.private_area
    }
    pub fn age_in_years(&self) -> &str {
        &self.age_in_years
    }
    pub fn construction_date_yyyymm(&self) -> &str {
        &self.construction_date_yyyymm
    }
    pub fn floor(&self) -> &str {
        &self.floor
    }
    pub fn number_of_floors(&self) -> &str {
        &self.number_of_floors
    }
    pub fn facing_direction(&self) -> &str {
        &self.facing_direction
    }
    pub fn building_type(&self) -> &str {
        &self.building_type
    }
    pub fn features(&self) -> &str {
        &self.features
    }
    pub fn structure(&self) -> &str {
        &self.structure
    }
    pub fn damage_insurance(&self) -> &str {
        &self.damage_insurance
    }
    pub fn parking(&self) -> &str {
        &self.parking
    }
    pub fn move_in(&self) -> &str {
        &self.move_in
    }
    pub fn transaction_type(&self) -> &str {
        &self.transaction_type
    }
    pub fn conditions(&self) -> &str {
        &self.conditions
    }
    pub fn property_code(&self) -> &str {
        &self.property_code
    }
    pub fn contract_period(&self) -> &str {
        &self.contract_period
    }
    pub fn notes(&self) -> &str {
        &self.notes
    }
    pub fn info_update_date(&self) -> DateTime<FixedOffset> {
        self.info_update_date
    }
    pub fn next_update_date(&self) -> DateTime<FixedOffset> {
        self.next_update_date
    }
    pub fn scraping_date(&self) -> DateTime<FixedOffset> {
        self.scraping_date
    }
    pub fn is_expired(&self) -> bool {
        self.is_expired
    }
}
