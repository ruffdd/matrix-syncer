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
use std::{fs, ops::Index};

use JsonValue::Array;
use json::JsonValue;
use json::parse;
use matrix_sdk::ruma::OwnedRoomId;
use matrix_sdk::ruma::api::client::user_directory::search_users::v3::User;
use matrix_sdk::ruma::events::policy::rule::user;
use matrix_sdk::ruma::{
    DeviceId, OwnedDeviceId, OwnedUserId, UserId, api::client::reporting::report_user,RoomId
};
#[derive(Clone)]
pub struct RoomMapping {
    pub user_ids: Vec<OwnedUserId>,
    pub room_id: OwnedRoomId,
}
#[derive(Clone)]
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
                room_id: RoomId::parse(get_string_value( mapping,"room-id")).expect("Could not convert to RoomId"),
                user_ids: mapping["user-ids"].members().map(|user_id|{
                    return UserId::parse(user_id.as_str().expect("")).expect("could no convert to user id");
                }).collect() 
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
