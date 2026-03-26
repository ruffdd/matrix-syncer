use anyhow::Ok;
use json;
use std::{fs, sync::Arc};

use matrix_sdk::{
    AuthSession, Client, Room, SessionMeta, SessionTokens,
    authentication::matrix::MatrixSession,
    config::SyncSettings,
    ruma::{
        OwnedRoomId, RoomId, UserId,
        events::{
            AnySyncStateEvent,
            call::invite::CallInviteEvent,
            room::{
                member::StrippedRoomMemberEvent,
                message::{OriginalSyncRoomMessageEvent, SyncRoomMessageEvent},
                third_party_invite::SyncRoomThirdPartyInviteEvent,
            },
        },
        presence::PresenceState,
    },
};

mod config;

fn room_name(room: &Room) -> String {
    return match room.name() {
        Some(o) => o,
        None => room.room_id().as_str().to_string(),
    };
}

async fn event_handler(ev: SyncRoomMessageEvent, room: Room, config: config::Config) {
    // println!("{}", room.room_id());
    for mapping in &config.mappings {
        if mapping.room_id != room.room_id() {
            println!("Found room {} which has no mapping", room_name(&room));
            continue;
        } else {
            for user in &mapping.user_ids {
                match room.invite_user_by_id(&user).await {
                    Err(e) => println!(
                        "Error ({}) while inviting {} into {} ",
                        e,
                        user,
                        room_name(&room)
                    ),
                    Result::Ok(()) => println!("Invited {} into {} ", user, room_name(&room)),
                }
            }
            break;
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::load_config();

    let client = Arc::new(
        Client::builder()
            .server_name(config.matrix_user_name.server_name())
            .build()
            .await?,
    );

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

    {
        let event_config = config.clone();
        client.add_event_handler(|event, room| event_handler(event, room, event_config));
    }
    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    let mut sync_settings = SyncSettings::new();
    sync_settings = sync_settings.full_state(true);
    sync_settings = sync_settings.set_presence(PresenceState::Offline);
    client.sync_once(sync_settings).await?;

    Ok(())
}
