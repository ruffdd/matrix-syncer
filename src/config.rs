use std::{fs, ops::Index};

use JsonValue::Array;
use json::JsonValue;
use json::parse;
use matrix_sdk::ruma::{
    DeviceId, OwnedDeviceId, OwnedUserId, UserId, api::client::reporting::report_user,
};

pub struct RoomMapping {
    room_id: String,
    user_ids: String,
}
pub struct Config {
    pub matrix_user_name: OwnedUserId,
    pub matrix_token: String,
    pub matrix_deviceID: OwnedDeviceId,
    pub mappings: Vec<RoomMapping>,
}

fn get_string_value(config_file: &JsonValue, path: &str) -> String {
    let mut value: &JsonValue = &config_file;
    path.split("/").for_each(|item| {
        print!("{}", value.to_string());
        value = &value[item];
    });
    return value
        .as_str()
        .expect("could not convert config value to string")
        .to_string();
}

pub fn load_config() -> Config {
    let config = json::parse(
        fs::read_to_string("./config.json")
            .expect("Could not load config")
            .as_str(),
    )
    .unwrap();

    let temp_mappings: Vec<RoomMapping> = config["mapping"]
        .members()
        .map(|mapping| {
            return RoomMapping {
                room_id: get_string_value( mapping,"room-id"),
                user_ids: get_string_value( mapping,"user-ids"),
            };
        })
        .collect();

    return Config {
        matrix_user_name: UserId::parse(get_string_value(&config, "matrix/username"))
            .expect("not a valid user id"),
        matrix_token: get_string_value(&config, "matrix/token"),
        matrix_deviceID: get_string_value(&config, "matrix/deviceID").into(),
        mappings: temp_mappings
    };
}
