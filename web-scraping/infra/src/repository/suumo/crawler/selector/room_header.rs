/// 賃貸一覧ページのセレクタ

pub fn residence_root() -> String {
    "div[class='cassetteitem']".to_string()
}
pub fn residence_title() -> String {
    "div[class='cassetteitem_content-title']".to_string()
}
pub fn residence_address() -> String {
    "li[class='cassetteitem_detail-col1']".to_string()
}
pub fn residence_nearest_station() -> String {
    "li[class='cassetteitem_detail-col2']".to_string()
}
pub fn residence_age() -> String {
    "li[class='cassetteitem_detail-col3'] div:nth-child(1)".to_string()
}
pub fn residence_floors() -> String {
    "li[class='cassetteitem_detail-col3'] div:nth-child(2)".to_string()
}
pub fn residence_transfer() -> String {
    "dd[class='cassetteitem_transfer-body'] li".to_string()
}
pub fn rooms() -> String {
    "table[class='cassetteitem_other'] tbody".to_string()
}
pub fn room_floor() -> String {
    "td:nth-child(3)".to_string()
}
pub fn room_rent_price() -> String {
    "span[class='cassetteitem_price cassetteitem_price--rent']".to_string()
}
pub fn room_condo_fee() -> String {
    "span[class='cassetteitem_price cassetteitem_price--administration']".to_string()
}
pub fn room_deposit() -> String {
    "span[class='cassetteitem_price cassetteitem_price--deposit']".to_string()
}
pub fn room_key_money() -> String {
    "span[class='cassetteitem_price cassetteitem_price--gratuity']".to_string()
}
pub fn room_layout() -> String {
    "span[class='cassetteitem_madori']".to_string()
}
pub fn room_exclusive_area() -> String {
    "span[class='cassetteitem_menseki']".to_string()
}
pub fn room_path() -> String {
    "a[href][class='js-cassette_link_href cassetteitem_other-linktext']".to_string()
}
pub fn pagination_parts() -> String {
    "ol[class='pagination-parts'] > li > a".to_string()
}
