use anyhow::Ok;
use json;
use std::{fs, sync::Arc};

use matrix_sdk::{
    AuthSession, Client, Room, SessionMeta, SessionTokens,
    authentication::matrix::MatrixSession,
    config::SyncSettings,
    ruma::{
        RoomId, UserId,
        events::{
            AnySyncStateEvent,
            call::invite::CallInviteEvent,
            room::{
                member::StrippedRoomMemberEvent,
                message::{OriginalSyncRoomMessageEvent, SyncRoomMessageEvent},
                third_party_invite::SyncRoomThirdPartyInviteEvent,
            },
        },
    },
};

mod config;

async fn event_handler(ev: SyncRoomMessageEvent, room: Room,config:Arc<config::Config>,client:Arc<Client>) {
    // println!("{}", room.room_id());
    for mapping in &config.mappings {
        for user in &mapping.user_ids {
            let room = client.get_room(&mapping.room_id);
            if room.is_none() {
                println!("could not get room {}", mapping.room_id.as_str())
            } else {
                room.unwrap().invite_user_by_id(&user);
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Arc::new(config::load_config());

    let client = Arc::new(Client::builder()
        .server_name(config.matrix_user_name.server_name())
        .build()
        .await?);

    let matrix_session = MatrixSession {
        meta: SessionMeta {
            user_id: config.matrix_user_name.clone(),
            device_id: config.matrix_deviceID.clone(),
        },
        tokens: SessionTokens {
            access_token: config.matrix_token.clone(),
            refresh_token: None,
        },
    };

    client
        .restore_session(AuthSession::Matrix(matrix_session))
        .await?;
    println!("logged in as: {}", client.user_id().expect(""));


    client.add_event_handler(| event,room| {
        event_handler(event, room,config.clone(),client.clone())
    });

    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    client.sync(SyncSettings::default()).await?;

    Ok(())
}
