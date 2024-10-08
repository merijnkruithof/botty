use std::sync::Arc;
use axum::Json;
use axum::routing::{delete, get, post, put};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tower_http::cors::CorsLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, ToSchema
};
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use crate::client::hotel;
use crate::core::taskmgr::task;

use crate::webapi::actions::web::WebService;
use crate::webapi::controller;
use crate::webapi::controller::bot::{bot_controller, friend_request, message_controller};
use crate::webapi::controller::bot::friend_request::broadcast_send_friend_request;
use crate::webapi::controller::hotel_controller;
use crate::webapi::controller::room::{enter_room_controller, room_dance_controller, walk_controller};

mod webapi;
mod app_config;
mod client;
mod communication;
mod retro;
pub mod connection;
mod core;
pub mod event;
pub mod room;
mod user;

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    #[derive(OpenApi, Debug)]
    #[openapi(
        modifiers(&SecurityAddon),
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-auth-token"))),
                )
            }
        }
    }

    let retro_manager = Arc::new(retro::Manager::new());
    let task_manager = Arc::new(task::Manager::new());

    let app_config = app_config::load().unwrap();

    let mut web_service = WebService::new();

    if app_config.use_default_handlers {
        for handler in app_config.handlers {
            let name = handler.name.clone();
            let ws_link = handler.ws_link.clone();
            let origin = handler.origin.clone();

            let retro_manager_clone = retro_manager.clone();

            tokio::spawn(async move {
                let connector = Arc::new(connection::Connector::new(ws_link, origin));
                let manager = Arc::new(hotel::Manager::new(connector));

                retro_manager_clone.add_hotel(name, manager).await.unwrap();
            });
        }
    }

    let openapi = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&openapi).unwrap();

    let mut file = File::create("openapi.json").await.unwrap();
    file.write_all(json.as_bytes()).await.unwrap();

    // Configure routes
    web_service.configure_routes(|router| {
        // Add Swagger UI
        let swagger_ui = SwaggerUi::new("/api-docs/swagger/")
            .url("/api-docs/openapi.json", ApiDoc::openapi());

        let router = router.merge(swagger_ui)
            .merge(Redoc::with_url("/api-docs/redoc/", ApiDoc::openapi()))
            .merge(RapiDoc::new("/api-docs/openapi.json").path("/api-docs/rapidoc"));

        // Health API
        let router = router.route("/api/health", get(webapi::health::index));

        // Task API
        let router = router.route("/api/tasks/delete", delete(webapi::task::kill_task));

        // Hotel API
        let router = router
            .route("/api/hotels", post(hotel_controller::add_hotel))
            .route("/api/hotels/:name", delete(hotel_controller::delete_hotel))
            .route("/api/hotels", get(hotel_controller::list));

        // Bot API
        let router = router
            .route("/api/hotels/:hotel/bots", get(bot_controller::index))
            .route("/api/hotels/:hotel/bots", put(bot_controller::bulk_update))
            .route("/api/hotels/:hotel/bots/:ticket", put(bot_controller::update))
            .route("/api/hotels/:hotel/bots/:ticket", get(bot_controller::show))
            .route("/api/hotels/:hotel/bots/:ticket/send_friend_request", post(friend_request::send_friend_request))
            .route("/api/hotels/:hotel/bots/send_friend_request", post(broadcast_send_friend_request));

        // Room API
        let router = router
            .route("/api/hotels/:hotel/rooms/enter", post(enter_room_controller::enter_room))
            .route("/api/hotels/:hotel/rooms/:room_id/walk_to_position", post(walk_controller::walk_to_position))
            .route("/api/hotels/:hotel/rooms/:room_id/walk_to_random_position", post(walk_controller::walk_randomly))
            .route("/api/hotels/:hotel/rooms/:room_id/dance", post(room_dance_controller::dance))
            .route("/api/hotels/:hotel/bots/message", post(message_controller::broadcast_message))
            .route("/api/hotels/:hotel/bots/enter_room", post(webapi::bot::broadcast_enter_room))
            .route("/api/hotels/:hotel/bots/cfh_abuse", post(webapi::bot::broadcast_cfh_abuse));

        // Session API
        let router = router
            .route("/api/hotels/:hotel/sessions", post(webapi::api_session::add))
            .route("/api/hotels/:hotel/sessions/add_many", post(webapi::api_session::add_many));

        return router;
    })?;

    // Configure webservice extensions
    web_service.configure_extensions(|router| {
        let app_state = webapi::app_state::AppState{
            auth_token: app_config.auth_token.clone(),
        };

        info!("Using auth token {}", &app_state.auth_token);

        return router
            .layer(axum::Extension(retro_manager))
            .layer(axum::Extension(task_manager))
            .layer(CorsLayer::permissive())
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), webapi::middleware::auth::handle));
    })?;

    web_service.start(app_config.webserver.clone()).await?;

    Ok(())
}
