use std::sync::Arc;
use axum::routing::{get, post};
use axum::extract::State;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;
use tower_http::cors::{CorsLayer};

mod api;
mod app_config;
mod client;
mod composer;
mod event;
mod packet;
mod session;
mod actions;
mod connection;

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ascii_art = r#" _  (`-') (`-')  _           (`-')  _  (`-').->            (`-').->
    \-.(OO ) ( OO).-/    .->    (OO ).-/  ( OO)_       .->    ( OO)_
    _.'    \(,------. ,---(`-') / ,---.  (_)--\_) ,--.(,--.  (_)--\_)
   (_...--'' |  .---''  .-(OO ) | \ /`.\ /    _ / |  | |(`-')/    _ /
   |  |_.' |(|  '--. |  | .-, \ '-'|_.' |\_..`--. |  | |(OO )\_..`--.
   |  .___.' |  .--' |  | '.(_/(|  .-.  |.-._)   \|  | | |  \.-._)   \
   |  |      |  `---.|  '-'  |  |  | |  |\       /\  '-'(_ .'\       /
   `--'      `------' `-----'   `--' `--' `-----'  `-----'    `-----'  "#;
    println!("{}", ascii_art);
    println!("Habbo Bot Commander - 2024 edition");
    println!("Developed by Merijn (Discord: merijnn)");
    println!("-------------------------------------------------------------------------------");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app_config = app_config::load().unwrap();
    let session_service = Arc::new(session::Service::new());

    let connection_service = Arc::new(connection::Service::new(
        connection::Config{ ws_link: app_config.uri, origin: app_config.origin },
        session_service.clone()
    ));

    let mut web_service = actions::web::WebService::new(666);

    // Configure routes
    web_service.configure_routes(|router| {
        // Health
        let router = router.route("/api/health", get(api::health::index));

        // Bot actions
        let router = router
            .route("/api/bots/available", get(api::bot::available))
            .route(
                "/api/bots/broadcast/message",
                post(api::bot::broadcast_message),
            )
            .route(
                "/api/bots/broadcast/enter_room",
                post(api::bot::broadcast_enter_room),
            );

        // Session actions
        let router = router
            .route("/api/sessions/add", post(api::api_session::add))
            .route("/api/sessions/add_many", post(api::api_session::add_many))
            .route("/api/sessions/kill", post(api::api_session::kill));

        return router;
    })?;

    // Configure webservice extensions
    web_service.configure_extensions(|router| {
        let app_state = api::app_state::AppState{
            auth_token: Uuid::new_v4().to_string(),
        };

        info!("Created auth token {}", &app_state.auth_token);

        return router
            .layer(axum::Extension(session_service))
            .layer(axum::Extension(connection_service))
            .layer(CorsLayer::permissive())
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), api::middleware::auth::handle));
    })?;

    web_service.start().await?;

    Ok(())
}