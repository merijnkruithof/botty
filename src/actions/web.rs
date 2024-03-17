use std::sync::Arc;
use anyhow::Result;
use axum::{middleware, Router};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Extension;
use tracing::info;
use uuid::Uuid;

pub struct WebService {
    router: Router,
}

impl WebService {
    pub fn new() -> Self {
        WebService{ router: Router::new() }
    }

    pub fn configure_routes<F>(&mut self, router: F) -> Result<()>
        where
            F: FnOnce(Router) -> Router
    {
        self.router = router(self.router.clone());

        Ok(())
    }

    pub fn configure_extensions<E>(&mut self, extensions: E) -> Result<()>
        where
            E: FnOnce(Router) -> Router
    {
        self.router = extensions(self.router.clone());

        Ok(())
    }

    pub async fn start(&mut self, connection_string: String) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(&connection_string).await?;

        info!("Webserver started on {}", &connection_string);

        axum::serve(listener, self.router.clone()).await.unwrap();

        Ok(())
    }
}