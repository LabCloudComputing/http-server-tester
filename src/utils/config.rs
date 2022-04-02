/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:11:24
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-02 22:40:06
 * @FilePath: /http-server-tester/src/utils/config.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::fs;

pub fn read_config_file(config_file: String) -> Result<serde_json::Value, String> {
    let file = match fs::File::open(config_file) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to read tester-config.json: {}", err));
        }
    };

    let config = match serde_json::from_reader(file) {
        Ok(config) => config,
        Err(err) => {
            return Err(format!("Failed to parse config file into JSON: {}", err));
        }
    };

    Ok(config)
}

pub fn get_json_value(config: &serde_json::Value, key: &str) -> Result<serde_json::Value, String> {
    let value = match config.get(key) {
        Some(value) => value,
        None => {
            return Err(format!("Can't find the key '{}' in config file.", key));
        }
    };

    Ok(value.clone())
}

pub fn get_json_value_as_u64(config: &serde_json::Value, key: &str) -> Result<u64, String> {
    let value = match config.get(key) {
        Some(value) => value,
        None => {
            return Err(format!("Config don't have the key '{}'.", key));
        }
    };

    match value.as_u64() {
        Some(value) => Ok(value),
        None => Err(format!(
            "The value of key '{}' is '{}' which should be an unsigned integer.",
            key, value
        )),
    }
}

pub fn get_json_value_as_string(config: &serde_json::Value, key: &str) -> Result<String, String> {
    let value = match config.get(key) {
        Some(value) => value,
        None => {
            return Err(format!("Config don't have the key '{}'.", key));
        }
    };

    match value.as_str() {
        Some(value) => Ok(value.to_string()),
        None => Err(format!(
            "The value of key '{}' is '{}' which should be a string.",
            key, value
        )),
    }
}

pub fn parse_server_args(config: &serde_json::Value) -> Result<(String, String), String> {
    let server = get_json_value(&config, "server")?;

    let ip = get_json_value_as_string(&server, "ip")?;
    let port = get_json_value_as_u64(&server, "port")?;
    let thread = get_json_value_as_u64(&server, "threads")?;

    Ok((
        format!("--ip {} --port {} --thread {}", ip, port, thread),
        format!("http://{}:{}", ip, port),
    ))
}
