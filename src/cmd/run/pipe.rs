/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-31 21:53:26
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-01 11:47:49
 * @FilePath: /http-server-tester/src/cmd/run/pipe.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */
use std::process;

use crate::utils::server;
use crate::Version;

pub fn pipelining(
    dir: &String,
    bin: &String,
    server_args: &String,
    base_url: &String,
    items: serde_json::Value,
    wait_seconds: u64,
    version: &Version,
) -> Result<(usize, usize), ()> {
    let (mut all, mut passes) = (0, 0);

    trace!("Testing pipelinging...");

    match items.as_array() {
        Some(pipes) => {
            let mut server = match version {
                Version::Debug => None,
                Version::Release => Some(server::try_run(dir, bin, &server_args, wait_seconds)),
            };

            for paths in pipes {
                match paths.as_array() {
                    Some(paths) => {
                        let ip_port = base_url.replace("http://", "");
                        let ip_port: Vec<&str> = ip_port.split(":").collect();
                        let ip = ip_port.get(0).unwrap();
                        let port = ip_port.get(1).unwrap();

                        let (cmd, paths) = parse_pipelining_requests(ip, port, paths)?;
                        let output = match process::Command::new("bash")
                            .arg("-c")
                            .arg(cmd.as_str())
                            .output()
                        {
                            Ok(output) => output,
                            Err(err) => {
                                error!("Running nc error: {}.", err);
                                return Err(());
                            }
                        };

                        let output = String::from_utf8_lossy(&output.stdout);
                        let output = parse_pipelining_response(output.as_ref());
                        let mut content = String::new();

                        let mut paths_str = String::from("[");
                        for path in paths {
                            paths_str = format!("{}{},", paths_str, path);
                            let cmd = format!("curl --connect-timeout 5 {}{}", &base_url, path);
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
                            let content_part = String::from_utf8_lossy(&output.stdout);
                            content = format!("{}{}", content, content_part);
                        }
                        paths_str = format!("{}\x08]", paths_str);

                        all += 1;
                        if content != output {
                            error!(
                                "The content of pipelining paths {} is different from single requests\n",
                                paths_str
                            );
                            debug!("You should recv:\n{}", content);
                            debug!("You actually recv:\n{}\n", output);
                        } else {
                            info!("Pipelining: paths - {} pass.", paths_str);
                            passes += 1;
                        }
                    }
                    None => {
                        error!("Pipe array is '{:?}', which should be an array.", paths);
                        server::try_kill(&mut server);
                        return Err(());
                    }
                }
                trace!("Testing pipe finished.");
                server::try_kill(&mut server);
            }

            Ok((all, passes))
        }

        None => {
            error!(
                "Pipelining item is '{:?}', which should be an array.",
                items
            );
            return Err(());
        }
    }
}

fn parse_pipelining_requests(
    ip: &str,
    port: &str,
    paths: &Vec<serde_json::Value>,
) -> Result<(String, Vec<String>), ()> {
    let mut all_requests = String::new();
    let mut all_paths = vec![];
    for path in paths {
        match path.as_str() {
            Some(path) => {
                all_requests = format!(
                    "{}GET {} HTTP/1.1\r\nHost: {}:{}\r\n\r\n",
                    all_requests, path, ip, port
                );
                all_paths.push(path.to_string());
            }
            None => {
                error!(
                    "The item of the array in pipelining is '{:?}', which should be a string.",
                    path
                );
                return Err(());
            }
        }
    }

    let cmd = format!("(echo -en \"{}\";)| nc {} {}", all_requests, ip, port);
    return Ok((cmd, all_paths));
}

fn parse_pipelining_response(response: &str) -> String {
    let bodys: Vec<&str> = response.split("\r\n\r\n").collect();
    let mut all_body = String::new();
    for body in bodys {
        let body = match body.find("HTTP/1.1") {
            Some(postion) => &body[..postion],
            None => body,
        };

        all_body = format!("{}{}", all_body, body);
    }

    all_body
}
