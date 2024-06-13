use wai_axum_extra::prelude::*;

/// The application level error
#[derive(thiserror::Error, Debug, Clone, ApplicationError)]
pub enum MyRustySandboxError {
    // TODO add your errors
}
