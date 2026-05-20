/* Matrix user Syncer
 * Copyright (C) 2025-2026  David Ruff develop@ruffdd.de
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use anyhow::{Error, Ok};
use json;
use std::{fmt::Debug, fs, sync::Arc};

use matrix_sdk::{
    AuthSession, Client, Room, SessionMeta, SessionTokens,
    authentication::matrix::MatrixSession,
    config::SyncSettings,
    ruma::{
        OwnedRoomId, OwnedUserId, RoomId, UserId, api::client::{account::change_password::v3::Request, membership::{self, invite_user::v3::InvitationRecipient}}, events::{
            AnySyncStateEvent,
            call::invite::CallInviteEvent,
            room::{
                member::StrippedRoomMemberEvent,
                message::{OriginalSyncRoomMessageEvent, SyncRoomMessageEvent},
                third_party_invite::SyncRoomThirdPartyInviteEvent,
            },
        }, presence::PresenceState
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

    for mapping in &config.mappings {
        for user in &mapping.user_ids {
            let request:membership::invite_user::v3::Request  = membership::invite_user::v3::Request::new(mapping.room_id.clone(),InvitationRecipient::UserId { user_id: user.clone() });
                    // ,
                    // reason:Option::Some( String::from("Automatic invited"))
                // };
            let response = client.send(request).await;
            let mut mes:String = String::from("");
            if(response.is_err()){
                mes = response.expect_err("").to_string();
            } 
            println!("Invited user {} into {}:{}", user.as_str(),mapping.room_id.as_str(),mes);
        }
    }


    Ok(())
}
