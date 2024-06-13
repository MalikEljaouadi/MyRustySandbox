use clap::Parser;
use wai_axum_extra::prelude::*;

/// `MyRustySandbox` application CLI arguments
#[derive(Debug, Clone, Parser)]
pub struct MyRustySandboxSettings {
    /// The server configuration
    #[clap(flatten)]
    pub server: ServerSettings,
    // Other settings ...
}
