use time::{OffsetDateTime, UtcOffset};

const SECONDS_IN_MINUTE: i32 = 60;


pub(crate) fn makeDateTime(inputTime: &git2::Time) -> OffsetDateTime
{
    let timeZoneOffset = UtcOffset::from_whole_seconds(inputTime.offset_minutes() * SECONDS_IN_MINUTE).unwrap();
    OffsetDateTime::from_unix_timestamp(inputTime.seconds()).unwrap().to_offset(timeZoneOffset)
}
