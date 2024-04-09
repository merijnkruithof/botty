use std::future::Future;
use std::sync::Arc;



use futures_util::SinkExt;


use tokio::sync::watch;
use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;

use tracing::{debug, error, info};


use crate::client::state;
use crate::communication::outgoing::composer;
use crate::communication::outgoing::composer::Composable;
use crate::core::events::connection::ReceiverEvent;
use crate::event::ControllerEvent;

pub struct Handler {
    global_state: Arc<state::GlobalState>,
    bot_state: Arc<state::BotState>,
}

impl Handler {
    pub fn new(
        global_state: Arc<state::GlobalState>,
        bot_state: Arc<state::BotState>
    ) -> Self {
        Handler {
            global_state,
            bot_state
        }
    }

    pub async fn start(&self) {
        info!("Starting communication handler for {}", &self.bot_state.session.ticket);

        let (kill_tx, kill_rx) = watch::channel(false);
        
        // Create listener of user manager
        let user_listener = self.create_user_listener(kill_rx.clone());
        let room_listener = self.create_room_listener(kill_rx.clone());
        let packet_listener = self.listen_to_packets(kill_tx);
        let ping_handle = self.handle_ping_pong(kill_rx.clone());
        let authentication_handle = self.handle_authentication_ok(kill_rx.clone());

        // Send authentication packets
        let _ = self.bot_state.packet_tx.send(composer::ClientHello{}.compose()).await;
        debug!("[C->S] Sent ClientHello for sso {}", &self.bot_state.session.ticket);

        let _ = self.bot_state.packet_tx.send(composer::AuthTicket{ sso_ticket: &*self.bot_state.session.ticket }.compose()).await;
        debug!("[C->S] Sent AuthTicket for sso {}", &self.bot_state.session.ticket);

        match tokio::try_join!(ping_handle, packet_listener, authentication_handle, user_listener, room_listener) {
            Ok(_) => info!("Tasks completed successfully"),
            Err(err) => error!("Unable to continue tasks: {:?}", err)
        };

        info!("Closed communication handler for sso {}", &self.bot_state.session.ticket);
    }

    fn listen_to_packets(&self, kill_tx: watch::Sender<bool>) -> JoinHandle<()> {
        let _sso = self.bot_state.session.ticket.clone();

        // hack to get mutable subscriber
        let mut receiver = self.bot_state.receiver.resubscribe();
        let event_handler = self.bot_state.event_handler.clone();
        let sso = self.bot_state.session.ticket.clone();

        return tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(data) => {
                        match data {
                            ReceiverEvent::MessageReceived { msg } => {
                                let event_handler_clone = event_handler.clone();
                                tokio::spawn(async move {
                                    let _ = event_handler_clone.handle(msg.into_data());
                                });
                            },

                            ReceiverEvent::ReceiverClosed => {
                                info!("Stopped packet listener for sso {}", &sso);
                                kill_tx.send(true).unwrap();
                            }
                        }

                    },

                    Err(err) => {
                        error!("Unable to receive packets anymore, killing listener. Reason: {:?}", err);
                        kill_tx.send(true).unwrap();
                        return;
                    },
                }
            }
        });
    }

    fn handle_ping_pong(&self, mut kill_rx: watch::Receiver<bool>) -> JoinHandle<()> {
        let mut subscriber = self.bot_state.event_handler.tx.subscribe();
        let packet_tx = self.bot_state.packet_tx.clone();
        let sso = self.bot_state.session.ticket.clone();

        return tokio::spawn(async move {
            loop {
                tokio::select! {
                    data = subscriber.recv() => {
                        let tx = packet_tx.clone();
                        tokio::spawn(async move {
                            if let Ok(ControllerEvent::Ping) = data {
                                let _ = tx.send(composer::Pong{}.compose()).await;
                            }
                        });
                    },

                    Ok(_) = kill_rx.changed() => {
                        if *kill_rx.borrow() {
                            info!("Closed ping pong listener for sso: {}", &sso);
                            return;
                        }
                    }
                }
            }
        });
    }
    fn handle_authentication_ok(&self, mut kill_rx: watch::Receiver<bool>) -> JoinHandle<()> {
        let mut subscriber = self.bot_state.event_handler.tx.subscribe();
        let sso = self.bot_state.session.ticket.clone();
        let packet_tx = self.bot_state.packet_tx.clone();

        return tokio::spawn(async move {
            loop {
                tokio::select! {
                    data = subscriber.recv() => {
                        let tx = packet_tx.clone();
                        tokio::spawn(async move {
                            if let Ok(ControllerEvent::AuthenticationOk) = data {
                                let _ = tx.send(composer::RequestUserData{}.compose()).await;
                                debug!("[C->S] Sent RequestUserData packet");
                            }
                        });
                    },

                    Ok(_) = kill_rx.changed() => {
                        if *kill_rx.borrow() {
                            info!("Closed authentication listener for sso {}", &sso);
                            return;
                        }
                    }
                }
            }
        });
    }
    fn create_user_listener(&self, kill_rx: Receiver<bool>) -> JoinHandle<()> {
        let user_manager = self.global_state.user_manager.clone();
        let event_rx = self.bot_state.event_handler.tx.subscribe();
        let session = self.bot_state.session.clone();

        tokio::spawn(async move {
            let _ = user_manager.listen(session, event_rx, kill_rx).await;
        })
    }
    fn create_room_listener(&self, kill_rx: Receiver<bool>) -> JoinHandle<()> {
        let room_manager = self.global_state.room_manager.clone();
        let event_tx = self.bot_state.event_handler.tx.subscribe();
        let bot_state = self.bot_state.clone();
        let global_state = self.global_state.clone();

        tokio::spawn(async move {
            let _ = room_manager.listen(global_state, bot_state, event_tx, kill_rx).await;
        })
    }
}