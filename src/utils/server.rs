/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:09:36
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-03-31 17:19:53
 * @FilePath: /http-server-tester/src/utils/server.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::process;
use std::thread;
use std::time::Duration;

fn run(dir: &String, bin: &String, server_args: &String) -> process::Child {
    let cmd = format!("{} {}", bin, server_args);

    let child = process::Command::new("bash")
        .current_dir(dir)
        .arg("-c")
        .arg(cmd.as_str())
        .spawn()
        .expect("Unknown error for running server...");
    child
}

pub fn try_run(
    dir: &String,
    bin: &String,
    server_args: &String,
    wait_seconds: u64,
) -> process::Child {
    let mut server = run(dir, bin, server_args);
    trace!("Waiting in {} seconds for server to start...", wait_seconds);
    thread::sleep(Duration::from_secs(wait_seconds));
    match server.try_wait() {
        Ok(Some(_)) => {
            error!("The server isn't running.");
            error!("Tester exited with errors.");
            process::exit(-1);
        }
        Ok(None) => {}
        Err(e) => println!("error attempting to wait: {}", e),
    }

    server
}

pub fn try_kill(server: &mut Option<process::Child>) {
    if let Some(server) = server {
        warn!("Trying to kill the HTTP Server...");
        server.kill().unwrap();
        trace!("The HTTP Server is stopped.");
    }
}
