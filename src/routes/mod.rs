use axum::Router;

use crate::service::MyRustySandboxState;

/// This is the application router
pub fn build_application_router() -> Router<MyRustySandboxState> {
    Router::new().nest("/ai/my-rusty-sandbox", Router::new())
}
