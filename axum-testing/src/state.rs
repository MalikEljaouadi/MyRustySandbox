use time::OffsetDateTime;

/// A dummy state, always UP
#[derive(Debug, Clone)]
pub struct DummyState {
    start_time: OffsetDateTime,
}

impl Default for DummyState {
    fn default() -> Self {
        let start_time = OffsetDateTime::now_utc();
        Self { start_time }
    }
}
