use axum::async_trait;
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

#[async_trait]
impl<E> ApplicationState<E> for DummyState
where
    E: ApplicationError,
{
    async fn check_health_status(&self) -> Result<HealthCheckStatus, E> {
        Ok(HealthCheckStatus::up("0.0.0", self.start_time))
    }
}