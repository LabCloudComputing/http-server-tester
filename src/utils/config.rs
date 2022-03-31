/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:11:24
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-01 00:33:12
 * @FilePath: /http-server-tester/src/utils/config.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::fs;
use std::process;

pub fn read_config_file(config_file: String) -> serde_json::Value {
    let file = match fs::File::open(config_file) {
        Ok(file) => file,
        Err(err) => {
            error!("Failed to read tester-config.json: {}", err);
            error!("Tester exited with errors.");
            process::exit(-1);
        }
    };

    let config = match serde_json::from_reader(file) {
        Ok(config) => config,
        Err(err) => {
            error!("Failed to parse config file into JSON: {}", err);
            error!("Tester exited with errors.");
            process::exit(-1);
        }
    };

    config
}

pub fn get_json_value(config: &serde_json::Value, key: &str) -> Result<serde_json::Value, ()> {
    let value = match config.get(key) {
        Some(value) => value,
        None => {
            error!("Can't find the key '{}' in config file.", key);
            return Err(());
        }
    };

    Ok(value.clone())
}

pub fn get_json_value_as_u64(config: &serde_json::Value, key: &str) -> Result<u64, ()> {
    let value = match config.get(key) {
        Some(value) => value,
        None => {
            error!("Config don't have the key '{}'.", key);
            return Err(());
        }
    };

    match value.as_u64() {
        Some(value) => Ok(value),
        None => {
            error!(
                "The value of key '{}' is '{}' which should be an unsigned integer.",
                key, value
            );
            Err(())
        }
    }
}

pub fn get_json_value_as_string(config: &serde_json::Value, key: &str) -> Result<String, ()> {
    let value = match config.get(key) {
        Some(value) => value,
        None => {
            error!("Config don't have the key '{}'.", key);
            return Err(());
        }
    };

    match value.as_str() {
        Some(value) => Ok(value.to_string()),
        None => {
            error!(
                "The value of key '{}' is '{}' which should be a string.",
                key, value
            );
            Err(())
        }
    }
}

pub fn parse_server_args(config: &serde_json::Value) -> Result<(String, String), ()> {
    let server = get_json_value(&config, "server")?;

    let ip = get_json_value_as_string(&server, "ip")?;
    let port = get_json_value_as_u64(&server, "port")?;
    let thread = get_json_value_as_u64(&server, "threads")?;

    Ok((
        format!("--ip {} --port {} --thread {}", ip, port, thread),
        format!("http://{}:{}", ip, port),
    ))
}
