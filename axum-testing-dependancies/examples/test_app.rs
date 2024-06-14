// Create a server for testing.

// The server is automatically stopped during dropping.
// The server is started on an available port.
// It also include an [`HttpClientForTest`] for testing routes.

use std::convert::Infallible;

use axum::routing::get;
use axum::Router;
use once_cell::sync::Lazy;
use wai_axum_extra::prelude::init_tracing;
use wai_axum_extra_test::TestApp;
use wai_axum_extra_test::DummyState;

// To avoid multiple tracing initialization
static INIT_TRACING: Lazy<()> = Lazy::new(|| {
    if let Err(err) = init_tracing() {
        eprintln!("Fail to initialize tracing because: {err:?}");
    }
});

// To avoid create your state and router on each tests.
// You can create multiple function if you need to have multiple application configurations
// e.g. start and initialize a DB
async fn build_test_app() -> TestApp {
    Lazy::force(&INIT_TRACING);
    let state = DummyState::default();
    let router = Router::new()
        .route("/", get(|| async move { axum::Json("🏡 Home") }));
    TestApp::start::<_, Infallible, _>(state, router).await
}

// A basic test on the home endpoint
#[test]
fn should_have_home() {
    let app = build_test_app();
    let result = app.get_json::<String>("/");
    assert_eq!(result, Ok("🏡 Home"));
}

// A basic test for an error
#[test]
fn should_404_on_bad_route() {
    let app = build_test_app();
    let result = app.get_json::<String>("/plaf");
    let (status, Some(err)) = result else { panic!("Should fail, got {result:?}"); };
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(err.code, "ai-not-found");
}
