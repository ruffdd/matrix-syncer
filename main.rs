use json;
use std::fs;
use url::Url;

use matrix_sdk::{
    Client, SessionMeta,
    authentication::matrix::MatrixSession,
    config::SyncSettings,
    ruma::{UserId, events::room::{encrypted::CiphertextInfo, message::SyncRoomMessageEvent}, user_id},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = json::parse(
        fs::read_to_string("./config.json")
            .expect("Should have been able to read the file")
            .as_str(),
    )
    .unwrap();
    // println!("{}",json::stringify(config.clone()));
    let redirect_url ="https://matrix.stuvus.uni-stuttgart.de/_synapse/client/oidc/callback";

    let user = UserId::parse(config["username"].as_str().expect("no username in config")).unwrap();
    let client = Client::builder()
        .server_name(user.server_name())
        .build()
        .await?;

    let callback_url = Url::parse(client.matrix_auth().get_sso_login_url(redirect_url, None).await?.as_str()).unwrap();

    println!("{}",callback_url.as_str());

    // First we need to log in.
    // client.matrix_auth().login_(config["token"].as_str().expect("No username provided in config")).send().await?;
    // let response = client.matrix_auth()
    //     .login_with_sso_callback(callback_url)
    //     .unwrap()
    //     .initial_device_display_name("My app")
    //     .await
    //     .unwrap();
    

    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });

    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    client.sync(SyncSettings::default()).await?;

    Ok(())
}
