use anyhow::Result;
use axum::Router;
use tracing::info;

pub struct WebService {
    pub port: usize,
    router: Router,
}

impl WebService {
    pub fn new(port: usize) -> Self {
        WebService{ port, router: Router::new() }
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

    pub async fn start(&self) -> Result<()> {
        let connection_str = format!("0.0.0.0:{}", self.port);

        let listener = tokio::net::TcpListener::bind(connection_str.clone()).await?;

        info!("Webserver started on {}", &connection_str);

        axum::serve(listener, self.router.clone()).await.unwrap();

        Ok(())
    }
}