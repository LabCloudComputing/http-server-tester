/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 23:44:10
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-01 00:17:35
 * @FilePath: /http-server-tester/src/cmd/run/proxy.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::process;

use super::compare_result;
use crate::utils::{config, server};
use crate::Version;

pub fn proxy(
    dir: &String,
    bin: &String,
    server_args: &String,
    base_url: &String,
    items: serde_json::Value,
    wait_seconds: u64,
    version: &Version,
) -> Result<(usize, usize), ()> {
    let (mut all, mut passes) = (0, 0);

    trace!("Testing proxy...");

    match items.as_array() {
        Some(proxys) => {
            for items in proxys {
                let host = config::get_json_value_as_string(&items, "host")?;
                let items = config::get_json_value(&items, "paths")?;
                let proxy_server_args = format!("{} --proxy {}", server_args, host);
                let mut server = match version {
                    Version::Debug => None,
                    Version::Release => {
                        Some(server::try_run(dir, bin, &proxy_server_args, wait_seconds))
                    }
                };

                match items.as_array() {
                    Some(paths) => {
                        for path_ in paths {
                            let path = match path_.as_str() {
                                Some(path) => path,
                                None => {
                                    error!(
                                        "The value of key 'path' is '{}' which should be a string.",
                                        path_
                                    );
                                    server::try_kill(&mut server);
                                    return Err(());
                                }
                            };
                            let cmd = format!("curl --connect-timeout 5 {}{}", base_url, path);
                            let output = match process::Command::new("bash")
                                .arg("-c")
                                .arg(cmd.as_str())
                                .output()
                            {
                                Ok(output) => output,
                                Err(err) => {
                                    error!("Running curl error: {}.", err);
                                    return Err(());
                                }
                            };
                            let output = String::from_utf8_lossy(&output.stdout);
                            let cmd = format!("curl --connect-timeout 5 {}{}", host, path);
                            let content = match process::Command::new("bash")
                                .arg("-c")
                                .arg(cmd.as_str())
                                .output()
                            {
                                Ok(output) => output,
                                Err(err) => {
                                    error!("Running curl error: {}.", err);
                                    return Err(());
                                }
                            };
                            let content = String::from_utf8_lossy(&content.stdout);

                            all += 1;
                            if let Ok(()) = compare_result(
                                "Proxy",
                                path,
                                host.as_str(),
                                content.as_ref(),
                                output.as_ref(),
                            ) {
                                passes += 1;
                            };
                        }
                    }
                    None => {
                        error!("Paths item is '{:?}', which should be an array.", items);
                        server::try_kill(&mut server);
                        return Err(());
                    }
                }
                trace!("Testing proxy finished.");
                server::try_kill(&mut server);
            }

            Ok((all, passes))
        }

        None => {
            error!("Proxy item is '{:?}', which should be an array.", items);
            return Err(());
        }
    }
}
