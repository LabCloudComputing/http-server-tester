/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:05:17
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-02 21:19:04
 * @FilePath: /http-server-tester/src/cmd/build.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::process;

pub fn build_server(dir: &String, cmd: &String) -> Result<(), String> {
    trace!("Building server...");
    let output = match process::Command::new("bash")
        .current_dir(dir.as_str())
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(format!("Run server build command error: {}.", err));
        }
    };

    if output.status.success() {
        trace!("Building server finished.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Building server error:\n{}", stderr))
    }
}
