use axum::{BoxError, Router};
use http::Method;
use std::fmt::Debug;
use std::ops::Deref;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tracing::info;


use crate::{HttpClientForTest, MetricsValues};


#[derive(Debug)]
pub struct TestApp {
    address: String,
    rest_client: HttpClientForTest,
    shutdown_tx: Option<oneshot::Sender<()>>,
    join: JoinHandle<()>,
    graceful_shutdown: bool,
}

impl TestApp {
    /// Create and start the server
    ///
    /// # Panics
    ///
    /// Will panic if we cannot create the [`TcpListener`]
    /// Will panic if we cannot create the HTTP client
    #[allow(clippy::unused_async)] // TODO remove the `async` when bump MSRV to 1.72
    pub async fn start<S, E, M>(state: S, router: Router<S>) -> Self
    where
        S: ApplicationState<E, M> + 'static,
        E: ApplicationError + 'static,
        M: Metrics + 'static,
    {
        // Choose an available port
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        // Start with a custom shutdown channel
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let join = tokio::spawn(async move {
            start(listener, state, router, shutdown_rx).await.unwrap();
        });

        // Create client
        let address = format!("http://localhost:{port}"); //DevSkim: ignore DS137138
        let rest_client = HttpClientForTest::new(&address);

        Self {
            address,
            rest_client,
            shutdown_tx: Some(shutdown_tx),
            join,
            graceful_shutdown: true,
        }
    }

    /// On drop, instead of graceful shutdown, aborts the server
    pub fn abort_on_drop(&mut self) {
        self.graceful_shutdown = false;
    }

    /// The server address
    #[must_use]
    pub fn address(&self) -> &str {
        self.address.as_ref()
    }

    /// Get the metrics
    ///
    /// # Errors
    ///
    /// Fail when the request fail or return bad metrics
    pub async fn get_metrics(&self) -> Result<MetricsValues, BoxError> {
        let response = self
            .rest_client
            .do_raw(Method::GET, "metrics", None)
            .await?;
        let body = response.text().await?;
        let encoded = body.parse()?;
        Ok(encoded)
    }
}

impl Deref for TestApp {
    type Target = HttpClientForTest;

    fn deref(&self) -> &Self::Target {
        &self.rest_client
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if self.graceful_shutdown {
            info!("🚮 Gracefully shutting down the server");
            if let Some(tx) = self.shutdown_tx.take() {
                tx.send(()).unwrap();
            }
        } else {
            info!("🚮 Aborting the server");
            self.join.abort();
        }
    }
}