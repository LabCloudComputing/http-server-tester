/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 23:43:23
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-17 19:48:03
 * @FilePath: /http-server-tester/src/cmd/run/http.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::fs;
use std::process;

use super::compare_result;
use crate::utils::{config, server};
use crate::Version;

pub fn http(
    dir: &String,
    bin: &String,
    server_args: &String,
    base_url: &String,
    mode: &String,
    items: serde_json::Value,
    wait_seconds: u64,
    version: &Version,
) -> Result<(usize, usize), String> {
    trace!("Testing http...");

    let mode_items = config::get_json_value(&items, mode.as_str())?;
    let get_items = config::get_json_value(&mode_items, "get")?;
    let post_items = config::get_json_value(&mode_items, "post")?;

    let mut server = match version {
        Version::Debug => None,
        Version::Release => Some(server::try_run(dir, bin, server_args, wait_seconds)?),
    };
    let get_result = match get(&base_url, &get_items) {
        Ok(result) => result,
        Err(err) => {
            server::try_kill(&mut server)?;
            return Err(err);
        }
    };

    let post_result = match post(mode.as_str(), &base_url, &post_items) {
        Ok(result) => result,
        Err(err) => {
            server::try_kill(&mut server)?;
            return Err(err);
        }
    };

    trace!("Testing HTTP finished.");
    server::try_kill(&mut server)?;

    Ok((get_result.0 + post_result.0, get_result.1 + post_result.1))
}

pub fn get(base_url: &str, items: &serde_json::Value) -> Result<(usize, usize), String> {
    let (mut all, mut passes) = (0, 0);
    match items.as_array() {
        Some(gets) => {
            for item in gets {
                let path = config::get_json_value_as_string(item, "path")?;
                let file = config::get_json_value_as_string(item, "file")?;

                let cmd = format!("curl --connect-timeout 5 \"{}{}\"", base_url, path);
                let output = match process::Command::new("bash")
                    .arg("-c")
                    .arg(cmd.as_str())
                    .output()
                {
                    Ok(output) => output,
                    Err(err) => {
                        return Err(format!("Running curl error: {}.", err));
                    }
                };

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Run curl failed:\n{}", stderr));
                }

                let output = String::from_utf8_lossy(&output.stdout);

                let content = match fs::read_to_string(file.clone()) {
                    Ok(content) => content,
                    Err(err) => {
                        return Err(format!("Failed to read file {}: {}", file, err));
                    }
                };

                all += 1;
                match compare_result(
                    "GET",
                    path.as_str(),
                    file.as_str(),
                    content.as_str(),
                    output.as_ref(),
                ) {
                    Ok(()) => passes += 1,
                    Err(()) => {}
                };
            }
        }

        None => {
            return Err(format!(
                "Get item is '{:?}', which should be an array.",
                items
            ));
        }
    }

    return Ok((all, passes));
}

pub fn post(
    mode: &str,
    base_url: &str,
    items: &serde_json::Value,
) -> Result<(usize, usize), String> {
    let (mut all, mut passes) = (0, 0);
    let posts = items.as_array();
    match posts {
        Some(posts) => {
            for item in posts {
                let path = config::get_json_value_as_string(item, "path")?;
                let payload = config::get_json_value_as_string(item, "payload")?;
                let file = config::get_json_value_as_string(item, "file")?;

                let payload = match fs::read_to_string(payload.clone()) {
                    Ok(content) => content,
                    Err(err) => {
                        return Err(format!("Failed to read payload file {}: {}", payload, err));
                    }
                };
                let cmd = match mode {
                    "basic" => format!("curl --connect-timeout 5 -d \"{}\" -X POST \"{}{}\"", payload, base_url, path),
                    "advanced" => format!(
                        "curl --connect-timeout 5 -H 'Content-Type: application/json' -d '{}' -X POST \"{}{}\"",
                        payload, base_url, path
                    ),
                    _ => String::new(),
                };
                let output = match process::Command::new("bash")
                    .arg("-c")
                    .arg(cmd.as_str())
                    .output()
                {
                    Ok(output) => output,
                    Err(err) => {
                        return Err(format!("Running curl error: {}.", err));
                    }
                };

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Run curl failed:\n{}", stderr));
                }

                let output = String::from_utf8_lossy(&output.stdout);

                let content = match fs::read_to_string(file.clone()) {
                    Ok(content) => content,
                    Err(err) => {
                        return Err(format!("Failed to read file {}: {}", file, err));
                    }
                };

                all += 1;
                match compare_result(
                    "POST",
                    path.as_str(),
                    file.as_str(),
                    content.as_ref(),
                    output.as_ref(),
                ) {
                    Ok(()) => passes += 1,
                    Err(()) => {}
                };
            }
        }

        None => {
            return Err(format!(
                "Post item is '{:?}', which should be an array.",
                items
            ));
        }
    }

    Ok((all, passes))
}
