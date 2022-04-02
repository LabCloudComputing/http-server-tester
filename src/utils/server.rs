/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:09:36
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-02 21:40:30
 * @FilePath: /http-server-tester/src/utils/server.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::process;
use std::thread;
use std::time::Duration;

fn run(dir: &String, bin: &String, server_args: &String) -> Result<process::Child, String> {
    let cmd = format!("{} {}", bin, server_args);

    match process::Command::new("bash")
        .current_dir(dir)
        .arg("-c")
        .arg(cmd.as_str())
        .spawn()
    {
        Ok(child) => Ok(child),
        Err(err) => Err(format!("Can't run your server: {}.", err)),
    }
}

pub fn try_run(
    dir: &String,
    bin: &String,
    server_args: &String,
    wait_seconds: u64,
) -> Result<process::Child, String> {
    let mut server = run(dir, bin, server_args)?;
    trace!("Waiting in {} seconds for server to start...", wait_seconds);
    thread::sleep(Duration::from_secs(wait_seconds));
    match server.try_wait() {
        Ok(Some(_)) => Err(format!("The server isn't running.")),
        Ok(None) => Ok(server),
        Err(err) => Err(format!("Can't wait server to run: {}.", err)),
    }
}

pub fn try_kill(server: &mut Option<process::Child>) -> Result<(), String> {
    match server {
        Some(server) => {
            warn!("Trying to kill the HTTP Server...");
            match server.kill() {
                Ok(()) => {
                    trace!("The HTTP Server is stopped.");
                    Ok(())
                }
                Err(err) => Err(format!("Kill the HTTP Server failed: {}.", err)),
            }
        }
        None => {
            warn!("The HTTP Server didn't run.");
            Ok(())
        }
    }
}
