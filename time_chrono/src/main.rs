use chrono::prelude::*;
use std::time::{UNIX_EPOCH, Duration};

fn main() {

    // utc and local
    {
        let sample_date1_in_utc: DateTime<Utc> = Utc::now();
        let sample_date2_in_utc: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 3, 1, 1, 2, 3).unwrap();
        let sample_date3_in_utc: DateTime<Utc> = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(1524885322));
        let sample_date4_in_utc: DateTime<Utc> = "2023-03-01 01:02:03 UTC".parse::<DateTime<Utc>>().unwrap();

        println!("sample date 1 (in utc): {sample_date1_in_utc}");
        println!("sample date 2 (in utc): {sample_date2_in_utc}");
        println!("sample date 3 (in utc): {sample_date3_in_utc}");
        println!("sample date 4 (in utc): {sample_date4_in_utc}");

        println!();

        let sample_date1_in_local_tz: DateTime<Local> = Local::now();
        let sample_date2_in_local_tz: DateTime<Local> = Local.with_ymd_and_hms(2023, 3, 1, 1, 2, 3).unwrap();
        let sample_date3_in_local_tz: DateTime<Local> = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs(1524885322));
        let sample_date4_in_local_tz: DateTime<Local> = "2023-03-01 01:02:03 UTC".parse::<DateTime<Local>>().unwrap();

        println!("sample date 1 (in local_tz): {sample_date1_in_local_tz}");
        println!("sample date 2 (in local_tz): {sample_date2_in_local_tz}");
        println!("sample date 3 (in local_tz): {sample_date3_in_local_tz}");
        println!("sample date 4 (in local_tz): {sample_date4_in_local_tz}");

        println!()
    }

    // naive date time
    {
        let timestamp_in_second: i64 = 1524885322;
        let naive_date_time: NaiveDateTime = NaiveDateTime::from_timestamp_millis(timestamp_in_second * 1000).unwrap();
        
        let sample_date_in_utc = Utc.from_utc_datetime(&naive_date_time);
        let sample_date_in_local_tz = Local.from_local_datetime(&naive_date_time).unwrap();
        
        println!("sample date 1 (in utc):      {sample_date_in_utc}");
        println!("sample date 2 (in local_tz): {sample_date_in_local_tz}");

        println!()
    }

    // utc and local conversion
    {
        let date1_in_local_tz: DateTime<Local> = Local::now();
        println!("date (in local): {date1_in_local_tz}");
        
        let date_in_utc = DateTime::<Utc>::from(date1_in_local_tz);
        println!("date (in utc):   {date_in_utc}");
        
        let date2_in_local_tz = DateTime::<Local>::from(date_in_utc);
        println!("date (in local): {date2_in_local_tz}");

        println!()
    }

    // date to timestamp
    {
        let date_in_local_tz: DateTime<Local> = Local::now();
        println!("date: {date_in_local_tz}");
        println!("date (in second timestamp):      {}", date_in_local_tz.timestamp());
        println!("date (in milisecond timestamp):  {}", date_in_local_tz.timestamp_millis());
        println!("date (in microsecond timestamp): {}", date_in_local_tz.timestamp_micros());

        let naive_date_time: NaiveDateTime = NaiveDateTime::from_timestamp_millis(date_in_local_tz.timestamp_millis()).unwrap();
        let new_datetime1 = Local.from_local_datetime(&naive_date_time).unwrap();
        println!("date1: {new_datetime1}");

        let new_datetime2: DateTime<Local> = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_millis(date_in_local_tz.timestamp_millis().unsigned_abs()));
        println!("date2: {new_datetime2}");

        println!()
    }

    // date formatting
    {
        let date1: DateTime<Local> = Local::now();
        println!("date1 (in local):  {}", date1);

        let str_from_date1 = date1.format("%Y-%m-%d %H:%M:%S %z").to_string();
        println!("date1 (in string): {}", str_from_date1);

        let date1_from_str = Local.datetime_from_str(&str_from_date1, "%Y-%m-%d %H:%M:%S %z").unwrap();
        println!("date1 (in local):  {}", date1_from_str);

        println!()
    }

    // date parsing
    {
        let date2_from_str = Utc.datetime_from_str("03/01/2023 13:04 +0000", "%m/%d/%Y %H:%M %z").unwrap();
        println!("date2 (in utc):    {}", date2_from_str);

        let str_from_date2 = date2_from_str.format("%Y-%m-%d %H:%M:%S %z").to_string();
        println!("date2 (in string): {}", str_from_date2);

        println!()
    }
}
