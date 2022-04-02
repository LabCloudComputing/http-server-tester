/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 21:46:18
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-04-02 22:42:54
 * @FilePath: /http-server-tester/src/cmd/run/perf.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::process;

use crate::utils::{config, server};
use crate::Version;

pub fn performance(
    dir: &String,
    bin: &String,
    server_args: &String,
    base_url: &String,
    items: serde_json::Value,
    wait_seconds: u64,
    version: &Version,
) -> Result<Vec<(usize, usize, f64, f64)>, String> {
    trace!("Testing performance...");

    match items.as_array() {
        Some(perf) => {
            let mut server = match version {
                Version::Debug => None,
                Version::Release => Some(server::try_run(dir, bin, server_args, wait_seconds)?),
            };

            let results = match run_ab(base_url, perf) {
                Ok(results) => results,
                Err(err) => {
                    server::try_kill(&mut server)?;
                    return Err(err);
                }
            };

            trace!("Testing performance finished.");
            server::try_kill(&mut server)?;
            Ok(results)
        }
        None => {
            return Err(format!(
                "Performance item is '{:?}', which should be an array.",
                items
            ));
        }
    }
}

pub fn run_ab(
    base_url: &String,
    perf: &Vec<serde_json::Value>,
) -> Result<Vec<(usize, usize, f64, f64)>, String> {
    let mut results = vec![];

    for item in perf {
        let path = config::get_json_value_as_string(item, "path")?;
        let requests = config::get_json_value_as_u64(item, "requests")?;
        let concurrency = config::get_json_value_as_u64(item, "concurrency")?;

        let cmd = format!("ab -n {} -c {} {}{}", requests, concurrency, base_url, path);
        let output = match process::Command::new("bash")
            .arg("-c")
            .arg(cmd.as_str())
            .output()
        {
            Ok(output) => output,
            Err(err) => {
                return Err(format!("Running ab error: {}.", err));
            }
        };

        if output.status.success() {
            let output = String::from_utf8_lossy(&output.stdout).as_ref().to_string();
            trace!("ab stdout output:\n{}", output);

            let (reqs_per_secs, time_per_reqs) = parse_ab_outout(output);
            results.push((
                requests as usize,
                concurrency as usize,
                reqs_per_secs,
                time_per_reqs,
            ));
        } else {
            let output = String::from_utf8_lossy(&output.stderr).as_ref().to_string();
            return Err(format!("ab test failed:\n {}.", output));
        }
    }

    Ok(results)
}

fn parse_ab_outout(output: String) -> (f64, f64) {
    let (mut reqs_per_secs, mut time_per_reqs) = (0.0, 0.0);

    let lines: Vec<&str> = output.split("\n").collect();

    for line in lines {
        if line.starts_with("Requests per second:") {
            reqs_per_secs = find_next_f64(line);
        }

        if line.starts_with("Time per request:") {
            time_per_reqs = find_next_f64(line);
        }
    }

    (reqs_per_secs, time_per_reqs)
}

fn find_next_f64(line: &str) -> f64 {
    let mut parts = line.split(" ");
    loop {
        let part = parts.next();
        if let Some(part) = part {
            if let Ok(num) = part.parse::<f64>() {
                break num;
            }
        } else {
            break 0.0;
        }
    }
}
