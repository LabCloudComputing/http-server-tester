/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:05:17
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-03-31 20:54:32
 * @FilePath: /http-server-tester/src/cmd/clean.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::process;

pub fn clean(dir: &String, cmd: &String) -> Result<(), ()> {
    trace!("Cleaning the project directory...");
    let output = match process::Command::new("bash")
        .current_dir(dir.as_str())
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            error!("Cleaning the project directory error: {}.", err);
            return Err(());
        }
    };

    if output.status.success() {
        trace!("Cleaning the project directory finished.");
        Ok(())
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        error!("Cleaning the project directory error.");
        trace!("Cleaning sdterr output:\n{}", stderr_str);
        Err(())
    }
}
