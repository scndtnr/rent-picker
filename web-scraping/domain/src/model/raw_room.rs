use chrono::{DateTime, FixedOffset};
use derive_new::new;

use super::Jst;

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawRoom {
    /// 詳細ページのURL
    url: String,
    /// リダイレクトされたURL
    redirect_url: String,
    /// suumo物件コード
    suumo_code: Option<String>,
    /// 建物名
    building_name: Option<String>,
    /// 家賃
    rental_fee: Option<String>,
    /// 管理費共益費
    management_fee: Option<String>,
    /// 敷金
    security_deposit: Option<String>,
    /// 礼金
    key_money: Option<String>,
    /// 保証金
    guarantee_deposit: Option<String>,
    /// 敷引償却
    key_money_amortization: Option<String>,
    /// 所在地
    location: Option<String>,
    /// 駅徒歩
    walk_to_station: Option<String>,
    /// 間取り
    floor_plan: Option<String>,
    /// 間取り詳細
    floor_plan_details: Option<String>,
    /// 専有面積
    private_area: Option<String>,
    /// 築年数
    age_in_years: Option<String>,
    /// 築年月
    construction_date_yyyymm: Option<String>,
    /// 階
    floor: Option<String>,
    /// 階建
    number_of_floors: Option<String>,
    /// 向き
    facing_direction: Option<String>,
    /// 建物種別
    building_type: Option<String>,
    /// 部屋の特徴設備
    features: Option<String>,
    /// 構造
    structure: Option<String>,
    /// 損保
    damage_insurance: Option<String>,
    /// 駐車場
    parking: Option<String>,
    /// 入居（時期）
    move_in: Option<String>,
    /// 取引態様
    transaction_type: Option<String>,
    /// 条件
    conditions: Option<String>,
    /// 取り扱い店舗物件コード
    property_code: Option<String>,
    /// 契約期間
    contract_period: Option<String>,
    /// 備考
    notes: Option<String>,
    /// 情報更新日
    info_update_date: Option<DateTime<FixedOffset>>,
    /// 次回更新日
    next_update_date: Option<DateTime<FixedOffset>>,
    /// スクレイピングした日時
    scraping_date: DateTime<FixedOffset>,
    /// 掲載終了フラグ
    is_expired: bool,
}

impl RawRoom {
    // new
    pub fn expired_new(url: &str, redirect_url: &str) -> Self {
        Self::new(
            url.to_string(),
            redirect_url.to_string(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Jst::now(),
            true,
        )
    }

    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn redirect_url(&self) -> &str {
        &self.redirect_url
    }
    pub fn suumo_code(&self) -> &Option<String> {
        &self.suumo_code
    }
    pub fn building_name(&self) -> &Option<String> {
        &self.building_name
    }
    pub fn rental_fee(&self) -> &Option<String> {
        &self.rental_fee
    }
    pub fn management_fee(&self) -> &Option<String> {
        &self.management_fee
    }
    pub fn security_deposit(&self) -> &Option<String> {
        &self.security_deposit
    }
    pub fn key_money(&self) -> &Option<String> {
        &self.key_money
    }
    pub fn guarantee_deposit(&self) -> &Option<String> {
        &self.guarantee_deposit
    }
    pub fn key_money_amortization(&self) -> &Option<String> {
        &self.key_money_amortization
    }
    pub fn location(&self) -> &Option<String> {
        &self.location
    }
    pub fn walk_to_station(&self) -> &Option<String> {
        &self.walk_to_station
    }
    pub fn floor_plan(&self) -> &Option<String> {
        &self.floor_plan
    }
    pub fn floor_plan_details(&self) -> &Option<String> {
        &self.floor_plan_details
    }
    pub fn private_area(&self) -> &Option<String> {
        &self.private_area
    }
    pub fn age_in_years(&self) -> &Option<String> {
        &self.age_in_years
    }
    pub fn construction_date_yyyymm(&self) -> &Option<String> {
        &self.construction_date_yyyymm
    }
    pub fn floor(&self) -> &Option<String> {
        &self.floor
    }
    pub fn number_of_floors(&self) -> &Option<String> {
        &self.number_of_floors
    }
    pub fn facing_direction(&self) -> &Option<String> {
        &self.facing_direction
    }
    pub fn building_type(&self) -> &Option<String> {
        &self.building_type
    }
    pub fn features(&self) -> &Option<String> {
        &self.features
    }
    pub fn structure(&self) -> &Option<String> {
        &self.structure
    }
    pub fn damage_insurance(&self) -> &Option<String> {
        &self.damage_insurance
    }
    pub fn parking(&self) -> &Option<String> {
        &self.parking
    }
    pub fn move_in(&self) -> &Option<String> {
        &self.move_in
    }
    pub fn transaction_type(&self) -> &Option<String> {
        &self.transaction_type
    }
    pub fn conditions(&self) -> &Option<String> {
        &self.conditions
    }
    pub fn property_code(&self) -> &Option<String> {
        &self.property_code
    }
    pub fn contract_period(&self) -> &Option<String> {
        &self.contract_period
    }
    pub fn notes(&self) -> &Option<String> {
        &self.notes
    }
    pub fn info_update_date(&self) -> Option<DateTime<FixedOffset>> {
        self.info_update_date
    }
    pub fn next_update_date(&self) -> Option<DateTime<FixedOffset>> {
        self.next_update_date
    }
    pub fn scraping_date(&self) -> DateTime<FixedOffset> {
        self.scraping_date
    }
    pub fn is_expired(&self) -> bool {
        self.is_expired
    }
}
