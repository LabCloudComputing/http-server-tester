/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-31 21:53:26
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-02 22:44:45
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
) -> Result<(usize, usize), String> {
    trace!("Testing pipelinging...");
    let (mut all, mut passes) = (0, 0);
    match items.as_array() {
        Some(pipes) => {
            let mut server = match version {
                Version::Debug => None,
                Version::Release => Some(server::try_run(dir, bin, &server_args, wait_seconds)?),
            };

            for paths in pipes {
                match paths.as_array() {
                    Some(paths) => {
                        let (cmd, paths) = parse_pipelining_requests(base_url, paths)?;

                        let output = match process::Command::new("bash")
                            .arg("-c")
                            .arg(cmd.as_str())
                            .output()
                        {
                            Ok(output) => output,
                            Err(err) => {
                                return Err(format!("Running nc error: {}.", err));
                            }
                        };

                        if !output.status.success() {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            return Err(format!("Run nc failed:\n{}", stderr));
                        }

                        let output = parse_pipelining_response(
                            String::from_utf8_lossy(&output.stdout).as_ref(),
                        );
                        let mut content = String::new();

                        let mut paths_str = String::new();
                        for path in paths {
                            paths_str = format!("{}{} ", paths_str, path);
                            let cmd = format!("curl --connect-timeout 5 {}{}", &base_url, path);
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
                                println!("{}", cmd);
                                return Err(format!("Run curl failed:\n{}", stderr));
                            }

                            let content_part = String::from_utf8_lossy(&output.stdout);
                            content = format!("{}{}", content, content_part);
                        }

                        all += 1;
                        if content != output {
                            error!(
                                "The content of pipelining paths {} is different from single requests\n",
                                paths_str
                            );
                            debug!("You should recv:\n{}", content);
                            debug!("You actually recv:\n{}\n", output);
                        } else {
                            info!("Pipelining: paths - {}pass.", paths_str);
                            passes += 1;
                        }
                    }
                    None => {
                        server::try_kill(&mut server)?;
                        return Err(format!(
                            "Pipe array is '{:?}', which should be an array.",
                            paths
                        ));
                    }
                }
            }
            trace!("Testing pipe finished.");
            server::try_kill(&mut server)?;
            Ok((all, passes))
        }

        None => {
            return Err(format!(
                "Pipelining item is '{:?}', which should be an array.",
                items
            ));
        }
    }
}

fn parse_pipelining_requests(
    base_url: &String,
    paths: &Vec<serde_json::Value>,
) -> Result<(String, Vec<String>), String> {
    let ip_port = base_url.replace("http://", "");
    let ip_port: Vec<&str> = ip_port.split(":").collect();
    let ip = ip_port.get(0).unwrap();
    let port = ip_port.get(1).unwrap();

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
                return Err(format!(
                    "The item of the array in pipelining is '{:?}', which should be a string.",
                    path
                ));
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
