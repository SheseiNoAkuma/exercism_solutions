use time::{OffsetDateTime, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let unix_timestamp = start.assume_utc().unix_timestamp() + 1_000_000_000;
    let offset_date =
        OffsetDateTime::from_unix_timestamp(unix_timestamp).expect("invalid unix timestamp");
    DateTime::new(offset_date.date(), offset_date.time())
}
