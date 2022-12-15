pub trait SuumoSelector {
    // トップページのセレクタ
    fn kanto_title(&self) -> &str {
        "h1[class='hantitle-txt']"
    }

    // 賃貸一覧ページのセレクタ

    fn residence_root(&self) -> &str {
        "div[class='cassetteitem']"
    }
    fn residence_title(&self) -> &str {
        "div[class='cassetteitem_content-title']"
    }
    fn residence_address(&self) -> &str {
        "li[class='cassetteitem_detail-col1']"
    }
    fn residence_nearest_station(&self) -> &str {
        "li[class='cassetteitem_detail-col2']"
    }
    fn residence_age(&self) -> &str {
        "li[class='cassetteitem_detail-col3'] div:nth-child(1)"
    }
    fn residence_floors(&self) -> &str {
        "li[class='cassetteitem_detail-col3'] div:nth-child(2)"
    }
    fn residence_transfer(&self) -> &str {
        "dd[class='cassetteitem_transfer-body'] li"
    }
    fn rooms(&self) -> &str {
        "table[class='cassetteitem_other'] tbody"
    }
    fn room_floor(&self) -> &str {
        "td:nth-child(3)"
    }
    fn room_rent_price(&self) -> &str {
        "span[class='cassetteitem_price cassetteitem_price--rent']"
    }
    fn room_condo_fee(&self) -> &str {
        "span[class='cassetteitem_price cassetteitem_price--administration']"
    }
    fn room_deposit(&self) -> &str {
        "span[class='cassetteitem_price cassetteitem_price--deposit']"
    }
    fn room_key_money(&self) -> &str {
        "span[class='cassetteitem_price cassetteitem_price--gratuity']"
    }
    fn room_layout(&self) -> &str {
        "span[class='cassetteitem_madori']"
    }
    fn room_exclusive_area(&self) -> &str {
        "span[class='cassetteitem_menseki']"
    }
    fn room_path(&self) -> &str {
        "a[href][class='js-cassette_link_href cassetteitem_other-linktext']"
    }
    fn pagination_parts(&self) -> &str {
        "ol[class='pagination-parts'] > li > a"
    }

    // 賃貸詳細ページのセレクタ
}
