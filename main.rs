use json;
use std::fs;

use matrix_sdk::{
    AuthSession, Client, Room, SessionMeta, SessionTokens, authentication::matrix::MatrixSession, config::SyncSettings, ruma::{ UserId, events::{AnySyncStateEvent, call::invite::CallInviteEvent, room::{member::StrippedRoomMemberEvent, message::{OriginalSyncRoomMessageEvent, SyncRoomMessageEvent}, third_party_invite::SyncRoomThirdPartyInviteEvent}}}
};

fn list_rooms(client:&Client){

    println!("Previously Invited Rooms:");
    for ele in client.rooms() {
        match ele.name(){
            Some(name) => print!("{} ",name),
            None => {}
        }
        println!("{},",ele.room_id());
    }
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = json::parse(
        fs::read_to_string("./config.json")
            .expect("Should have been able to read the file")
            .as_str(),
    )
    .unwrap();
    
    let user = UserId::parse(config["username"].as_str().expect("no username in config")).unwrap();
    let device= config["deviceID"].as_str().expect("No Device ID found in config").into();
    let access_token:String= config["token"].as_str().expect("Could not get token from config").to_string();
    let client = Client::builder()
        .server_name(user.server_name())
        .build()
        .await?;

    let matrix_session = MatrixSession{
        meta: SessionMeta { user_id: user, device_id: device },
        tokens: SessionTokens { access_token: access_token, refresh_token: None }
    };

    client.restore_session(AuthSession::Matrix( matrix_session)).await?;
    println!("logged in as: {}",config["username"]);

    client.add_event_handler(|ev: SyncRoomMessageEvent,room:Room| async move {
        println!("{}",room.room_id());
    });


    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    client.sync(SyncSettings::default()).await?;

    Ok(())
}
