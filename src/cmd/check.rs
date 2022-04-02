/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:02:28
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-02 21:17:34
 * @FilePath: /http-server-tester/src/cmd/check.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::process;

fn check_tool_version(tool: &str) -> bool {
    let cmd = format!("which {}", tool);
    let output = match process::Command::new("bash")
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            error!("Run the check command error: {}.", err);
            return false;
        }
    };

    if output.status.success() {
        true
    } else {
        error!("{} haven't been installed.", tool);
        false
    }
}

pub fn check_tools() -> Result<(), String> {
    trace!("Checking tools...");

    let curl = check_tool_version("curl");
    let nc = check_tool_version("nc");
    let ab = check_tool_version("ab");

    if curl && nc && ab {
        trace!("Checking tools finished.");
        return Ok(());
    }

    Err(format!(
        "Checking tools finished. Please install all tools."
    ))
}

pub fn check_mode(mode: &String) -> Result<(), String> {
    check_tools()?;
    match mode.as_str() {
        "basic" => Ok(()),
        "advanced" => Ok(()),
        _ => Err(format!(
            "Unknown mode {}, which should be 'basic' or 'advanced'.",
            mode
        )),
    }
}
