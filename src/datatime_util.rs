
pub fn instant_to_naive_datetime(instant: std::time::Instant) -> hbb_common::chrono::NaiveDateTime {
    let duration_since_epoch = instant.duration_since(std::time::Instant::now());
    let system_now = std::time::SystemTime::now() + duration_since_epoch;
    let utc_datetime: hbb_common::chrono::DateTime<hbb_common::chrono::Utc> = system_now.into();
    utc_datetime.naive_utc()
}
pub fn now_timestamp() -> i64 {
    let now = hbb_common::chrono::Utc::now();
    let st = now.timestamp_nanos_opt();
    st.unwrap()
}
pub fn timestamp_to_datetime(ts: i64) -> hbb_common::chrono::DateTime<hbb_common::chrono::Utc> {
    hbb_common::chrono::DateTime::from_timestamp_nanos(ts)
}

pub fn timestamp_to_naivedatetime(ts: i64) -> hbb_common::chrono::NaiveDateTime {
    hbb_common::chrono::DateTime::from_timestamp_nanos(ts).naive_local()
}

pub fn timestamp_to_millis(ts: i64) -> i64 {
    ts / 1000_000
}

pub trait nanos_to_millis {
    fn nanos_to_millis(self) -> i64;
}

impl nanos_to_millis for i64 {
    fn nanos_to_millis(self) -> i64 {
        timestamp_to_millis(self)
    }
}