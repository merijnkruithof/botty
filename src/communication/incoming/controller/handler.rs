use std::sync::Arc;
use anyhow::{anyhow, Result};
use crate::client::session::Session;
use crate::communication::incoming::controller;
use crate::communication::incoming::messages::Messages;
use crate::communication::packet::Reader;

pub trait Handler {
    async fn handle(&self, session: Arc<Session>, reader: Reader) -> Result<()>;
}

pub struct Factory { }

impl Factory {
    pub fn new() -> Self {
        Factory {}
    }

    pub fn make_controller(&self, header: u16) -> Result<Controller> {
        return match(header) {
            Messages::PING => Ok(Controller::Ping(controller::ping::PingHandler { })),
            _ => Err(anyhow!("No controller found for header {}", header))
        }
    }
}

pub enum Controller {
    Ping(controller::ping::PingHandler)
}

impl Controller {
    // return match header {
//     PING => tx.send(composer::Pong {}.compose()).await.unwrap(),
//
//     ROOM_READY => {
//         tx.send(composer::RequestRoomHeightmap {}.compose()).await.unwrap();
//
//         // useEffect(() =>
//         //  {
//         //  SendMessageComposer(new GetRoomEntryTileMessageComposer()); (FloorPlanEditorRequestDoorSettings)
//         //  SendMessageComposer(new GetOccupiedTilesMessageComposer()) (FloorPlanEditorRequestBlockedTiles).
//         //
//         //   Can be pretty useful later tbh
//         //  ... }
//         //
//         tx.send(composer::FloorPlanEditorRequestDoorSettings{}.compose()).await.unwrap();
//         tx.send(composer::FloorPlanEditorRequestBlockedTiles{}.compose()).await.unwrap();
//
//         debug!("Handled packet RoomReady (id: 2031)");
//     },
//
//     ROOM_INFO_OWNER => tx.send(composer::RequestRoomData{  }).unwrap()
//
//     _ => {
//         // do nothing
//     }
// }
    pub async fn handle(&self, session: Arc<Session>, reader: Reader) -> Result<()> {
        match self {
            Controller::Ping(handler) => handler.handle(session, reader).await
        }
    }
}