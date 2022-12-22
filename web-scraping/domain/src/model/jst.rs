use anyhow::{bail, Ok, Result};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Jst;

impl Jst {
    pub fn offset() -> FixedOffset {
        let hour = 60 * 60;
        let jst_offset = 9 * hour;
        FixedOffset::east_opt(jst_offset).unwrap()
    }

    pub fn now() -> DateTime<FixedOffset> {
        let now_utc = Utc::now().naive_utc();
        Self::offset().from_utc_datetime(&now_utc)
    }

    pub fn today() -> NaiveDate {
        Self::now().date_naive()
    }

    fn ymd(year: i32, month: u32, day: u32) -> NaiveDate {
        Self::offset()
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .unwrap()
            .date_naive()
    }

    pub fn from_utc_datetime(dt: &NaiveDateTime) -> DateTime<FixedOffset> {
        Jst::offset().from_utc_datetime(dt)
    }

    pub fn from_local_datetime(dt: &NaiveDateTime) -> DateTime<FixedOffset> {
        Jst::offset().from_local_datetime(dt).unwrap()
    }

    /// yyyy-mm-dd あるいは yyyy/mm/ddをDateTime<FixedOffset>オブジェクトに変換する
    pub fn datetime_from_date_str(
        s: &str,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> Result<DateTime<FixedOffset>> {
        let naive_date_time = Self::date_from_str(s)
            .unwrap()
            .and_hms_opt(hour, min, sec)
            .unwrap();
        match Self::offset()
            .from_local_datetime(&naive_date_time)
            .single()
        {
            Some(dt) => Ok(dt),
            None => bail!("Fail to convert from naive date time to JST"),
        }
    }

    pub fn datetime_from_dt_str(s: &str, fmt: &str) -> Result<DateTime<FixedOffset>> {
        match Self::offset().datetime_from_str(s, fmt).ok() {
            Some(dt) => Ok(dt),
            None => bail!("Invalid format. It must be '{}'. Input is '{}'", fmt, s),
        }
    }

    /// yyyy-mm-dd あるいは yyyy/mm/ddをdateオブジェクトに変換する
    /// 時分秒を追加する時は .and_hms_opt()を利用する
    pub fn date_from_str(s: &str) -> Result<NaiveDate> {
        let re = Regex::new(r"^(?P<y>\d{4})[/-年](?P<m>\d{1,2})[/-月](?P<d>\d{1,2})").unwrap();
        match re.captures(s) {
            Some(caps) => {
                let year = caps["y"].parse::<i32>().unwrap();
                let month = caps["m"].parse::<u32>().unwrap();
                let day = caps["d"].parse::<u32>().unwrap();
                Ok(Self::ymd(year, month, day))
            }
            None => bail!("Invalid format. It must be 'YYYY/MM/DD'. Input is '{}'", s),
        }
    }

    /// yyyymmdd に変換する場合："%Y%m%d"
    pub fn str_from_datetime(dt: DateTime<FixedOffset>, fmt: &str) -> String {
        dt.format(fmt).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_from_str() {
        let s = "2022/04/26 19:55";
        let date = Jst::date_from_str(s).unwrap();
        assert_eq!(date.to_string(), "2022-04-26");
    }

    #[test]
    fn datetime_from_str() {
        let s = "2022/04/26 19:55";
        let fmt = "%Y/%m/%d %H:%M";
        let dt = Jst::datetime_from_dt_str(s, fmt).unwrap();
        assert_eq!(dt.to_string(), "2022-04-26 19:55:00 +09:00");
    }

    #[test]
    fn date_with_hms_under_datetime() {
        let s = "2022/04/26 19:55";
        let fmt = "%Y/%m/%d %H:%M";

        // 各オブジェクトの作成
        let date = Jst::date_from_str(s).unwrap();
        let dt_from_date = date.and_hms_opt(0, 0, 0).unwrap();
        let dt = Jst::datetime_from_dt_str(s, fmt).unwrap();

        // DateとしてパースしたものとDatetimeとしてパースしたものの比較
        // Datetimeの方が時分秒の分、大きい値になるはず
        assert!(Jst::from_local_datetime(&dt_from_date) < dt);
    }

    #[test]
    fn str_from_datetime() {
        let s = "2022/04/26 19:55";
        let fmt_from = "%Y/%m/%d %H:%M";
        let fmt_to = "%Y/%m/%d %H時%M分%S秒";

        // Datetimeオブジェクト作成
        let dt = Jst::datetime_from_dt_str(s, fmt_from).unwrap();
        assert_eq!(dt.to_string(), "2022-04-26 19:55:00 +09:00");

        // 文字列に戻す
        assert_eq!(
            Jst::str_from_datetime(dt, fmt_to),
            "2022/04/26 19時55分00秒"
        )
    }
}
