/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:05:17
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-01 00:21:58
 * @FilePath: /http-server-tester/src/cmd/build.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::process;

pub fn build_server(dir: &String, cmd: &String) -> Result<(), ()> {
    trace!("Building server...");
    let output = match process::Command::new("bash")
        .current_dir(dir.as_str())
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            error!("Building server error: {}.", err);
            return Err(());
        }
    };

    if output.status.success() {
        trace!("Building server finished.");
        Ok(())
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        error!("Building server error.");
        trace!("Building sdterr output:\n{}", stderr_str);
        Err(())
    }
}
