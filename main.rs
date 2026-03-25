use json;
use std::fs;

use matrix_sdk::{
    AuthSession, Client, Room, SessionMeta, SessionTokens,
    authentication::matrix::MatrixSession,
    config::SyncSettings,
    ruma::{
        UserId,
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

async fn event_handler(ev: SyncRoomMessageEvent, room: Room) {
    println!("{}", room.room_id());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::load_config();

    let client = Client::builder()
        .server_name(config.matrix_user_name.server_name())
        .build()
        .await?;

    let matrix_session = MatrixSession {
        meta: SessionMeta {
            user_id: config.matrix_user_name,
            device_id: config.matrix_deviceID,
        },
        tokens: SessionTokens {
            access_token: config.matrix_token,
            refresh_token: None,
        },
    };

    client
        .restore_session(AuthSession::Matrix(matrix_session))
        .await?;
    println!("logged in as: {}", client.user_id().expect(""));

    client.add_event_handler(event_handler);

    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    client.sync(SyncSettings::default()).await?;

    Ok(())
}
