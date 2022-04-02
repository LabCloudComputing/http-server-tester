/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:05:17
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-02 22:36:13
 * @FilePath: /http-server-tester/src/cmd/clean.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::process;

pub fn clean(dir: &String, cmd: &String) -> Result<(), String> {
    trace!("Cleaning the project directory...");
    let output = match process::Command::new("bash")
        .current_dir(dir.as_str())
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(format!("Run the clean command error: {}.", err));
        }
    };

    if output.status.success() {
        trace!("Cleaning the project directory finished.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Cleaning the project directory error:\n{}", stderr))
    }
}
