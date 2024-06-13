use time::OffsetDateTime;
use wai_axum_extra::prelude::*;

use super::MyRustySandboxError;

/// Defining the application Axum state
#[derive(Debug, Clone)]
pub struct MyRustySandboxState {
    start_time: OffsetDateTime,
}

/// Should provide the health check
#[axum::async_trait]
impl ApplicationState<MyRustySandboxError> for MyRustySandboxState {
    async fn check_health_status(&self) -> Result<HealthCheckStatus, MyRustySandboxError> {
        let version = env!("CARGO_PKG_VERSION");
        Ok(HealthCheckStatus::up(version, self.start_time))
    }
}

/// Just a way to create the state
impl Default for MyRustySandboxState {
    fn default() -> Self {
        Self {
            start_time: OffsetDateTime::now_utc(),
        }
    }
}
