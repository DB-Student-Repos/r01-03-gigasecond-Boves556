use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond_duration = Duration::seconds(1_000_000_000);
    start + gigasecond_duration
}