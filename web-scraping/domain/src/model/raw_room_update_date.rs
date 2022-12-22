use chrono::{DateTime, Duration, FixedOffset};
use regex::Regex;

use super::Jst;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawRoomUpdateDate {
    info_update_date: DateTime<FixedOffset>,
    next_update_date: DateTime<FixedOffset>,
}

impl RawRoomUpdateDate {
    pub fn new(info_update_str: String, next_update_str: String) -> Self {
        let info_update_date = Jst::datetime_from_date_str(&info_update_str, 9, 0, 0).unwrap();

        let re = Regex::new(r"(?P<days>\d+)").unwrap();
        let next_update_str = Self::full_width_number_to_half_size(next_update_str);
        let next_update_days = match re.captures(&next_update_str) {
            Some(caps) => caps["days"].parse().unwrap(),
            None => panic!("Fail to parse next_update_str."),
        };
        let next_update_date = info_update_date + Duration::days(next_update_days);

        Self {
            info_update_date,
            next_update_date,
        }
    }
    pub fn info_update_date(&self) -> DateTime<FixedOffset> {
        self.info_update_date
    }
    pub fn next_update_date(&self) -> DateTime<FixedOffset> {
        self.next_update_date
    }

    /// 全角数字を半角数字に変換
    fn full_width_number_to_half_size(s: String) -> String {
        s.replace('０', "0")
            .replace('１', "1")
            .replace('２', "2")
            .replace('３', "3")
            .replace('４', "4")
            .replace('５', "5")
            .replace('６', "6")
            .replace('７', "7")
            .replace('８', "8")
            .replace('９', "9")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 問題なくnewできる() {
        let info_update_str = "2022/12/21".to_string();
        let next_update_str = "次回更新日は情報更新日より１５日以内".to_string();
        let raw_room_update_date = RawRoomUpdateDate::new(info_update_str, next_update_str);
        dbg!(raw_room_update_date);
    }
}
