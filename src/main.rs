use std::sync::Arc;


use axum::routing::{delete, get, post, put};
use tower_http::cors::CorsLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
    let ascii_art = r#" _  (`-') (`-')  _           (`-')  _  (`-').->            (`-').->
    \-.(OO ) ( OO).-/    .->    (OO ).-/  ( OO)_       .->    ( OO)_
    _.'    \(,------. ,---(`-') / ,---.  (_)--\_) ,--.(,--.  (_)--\_)
   (_...--'' |  .---''  .-(OO ) | \ /`.\ /    _ / |  | |(`-')/    _ /
   |  |_.' |(|  '--. |  | .-, \ '-'|_.' |\_..`--. |  | |(OO )\_..`--.
   |  .___.' |  .--' |  | '.(_/(|  .-.  |.-._)   \|  | | |  \.-._)   \
   |  |      |  `---.|  '-'  |  |  | |  |\       /\  '-'(_ .'\       /
   `--'      `------' `-----'   `--' `--' `-----'  `-----'    `-----'  "#;
    println!("{}", ascii_art);
    println!("Pegasus Server");
    println!("Developed by Merijn (Discord: merijnn)");
    println!("-------------------------------------------------------------------------------");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

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

                info!("Added hotel");

                retro_manager_clone.add_hotel(name, manager).await?;
            });
        }
    }

    // Configure routes
    web_service.configure_routes(|router| {
        let router = router.route("/api/health", get(webapi::health::index));
        let router = router.route("/api/tasks/delete", delete(webapi::task::kill_task));

        let router = router
            .route("/api/bots", post(bot_controller::index))
            .route("/api/bots/bulk_update", put(bot_controller::bulk_update))
            .route("/api/bots/:ticket", post(bot_controller::show))
            .route("/api/bots/:ticket", put(bot_controller::update))
            .route("/api/bots/:ticket/send_friend_request", post(friend_request::send_friend_request))
            .route("/api/bots/broadcast/send_friend_request", post(broadcast_send_friend_request))
            .route("/api/rooms/enter", post(enter_room_controller::enter_room))
            .route("/api/rooms/:room_id/walk_to_position", post(walk_controller::walk_to_position))
            .route("/api/rooms/:room_id/walk_to_random_position", post(walk_controller::walk_randomly))
            .route("/api/rooms/:room_id/dance", post(room_dance_controller::dance))
            .route("/api/bots/broadcast/message", post(message_controller::broadcast_message))
            .route("/api/bots/broadcast/enter_room", post(webapi::bot::broadcast_enter_room))
            .route("/api/bots/broadcast/cfh_abuse", post(webapi::bot::broadcast_cfh_abuse));

        let router = router
            .route("/api/hotels", post(hotel_controller::list));

        // Session actions
        let router = router
            .route("/api/sessions/add", post(webapi::api_session::add))
            .route("/api/sessions/add_many", post(webapi::api_session::add_many));

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
