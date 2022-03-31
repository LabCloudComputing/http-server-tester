/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:02:28
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-01 00:58:18
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
            error!("Checking tools error: {}.", err);
            return false;
        }
    };

    if output.status.success() {
        true
    } else {
        warn!("{} haven't been installed or run error.", tool);
        false
    }
}

pub fn check_tools() -> Result<(), ()> {
    trace!("Checking tools...");

    let curl = check_tool_version("curl");
    let nc = check_tool_version("nc");
    let ab = check_tool_version("ab");

    trace!("Checking tools finished.");

    if !curl {
        error!("Please installed curl.");
    }

    if !nc {
        error!("Please installed nc.");
    }

    if !ab {
        error!("Please installed ab.");
    }

    if curl && nc && ab {
        return Ok(());
    }

    Err(())
}

pub fn check_mode(mode: &String) -> Result<(), ()> {
    check_tools()?;
    match mode.as_str() {
        "basic" => Ok(()),
        "advanced" => Ok(()),
        _ => {
            error!(
                "Unknown mode {}, which should be 'basic' or 'advanced'.",
                mode
            );
            Err(())
        }
    }
}
