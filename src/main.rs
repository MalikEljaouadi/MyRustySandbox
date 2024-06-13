use axum::BoxError;
use clap::Parser;
use my_rusty_sandbox::launch;
use my_rusty_sandbox::settings::MyRustySandboxSettings;
use wai_axum_extra::prelude::*;

/// Run the server with a Tokio runtime
#[tokio::main]
async fn main() -> Result<(), BoxError> {
    // Initialize the tracing
    init_tracing()?;

    // Parsing args
    let settings = MyRustySandboxSettings::parse();

    // Let's go
    launch(settings).await
}
