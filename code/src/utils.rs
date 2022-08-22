use chrono::{prelude::*, format::{DelayedFormat, StrftimeItems}};

pub fn format_date_time(unix_timestamp: u64, format: &str) -> DelayedFormat<StrftimeItems>
{
    let naive_date_time = NaiveDateTime::from_timestamp(unix_timestamp as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_date_time, Utc);
    
    return datetime.format(format);
}