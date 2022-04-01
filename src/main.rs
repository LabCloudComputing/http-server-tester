mod cmd;
mod utils;

use cmd::{build, check, clean, run};
use utils::config;

#[macro_use]
extern crate log;
use clap::{Parser, Subcommand};
use log4rs;

use std::process;

/// Lab 2 HTTP Server Tester
#[derive(Parser)]
#[clap(author, version, about = "A CLI test program for HNU Cloud Computing Lab 2, built with Rust.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Check if tools have been installed
    Check,
    /// Build the http server
    Build,
    /// Clean the project directory
    Clean,
    /// Test the running http server
    Dev {
        #[clap(short, long, default_value = "basic")]
        /// Test in basic or advanced mode
        mode: String,
    },
    /// Test the http server including rebuilding and starting
    Run {
        #[clap(short, long, default_value = "basic")]
        /// Test in basic or advanced mode
        mode: String,
    },
}

pub enum Version {
    Debug,
    Release,
}

fn run_cmd() -> Result<(), ()> {
    match log4rs::init_file("config/log-config.yaml", Default::default()) {
        Ok(()) => {}
        Err(_) => {
            error!("Parse ./config/log-config.yaml failed, use default setting.")
        }
    };

    let args = Cli::parse();
    let config_file = "config/tester-config.json".to_string();
    let config = config::read_config_file(config_file);

    match &args.command {
        Command::Check => {
            check::check_tools()?;
        }
        Command::Build => {
            let dir = config::get_json_value_as_string(&config, "directory")?;
            let build = config::get_json_value_as_string(&config, "build")?;
            build::build_server(&dir, &build)?;
        }
        Command::Clean => {
            let dir = config::get_json_value_as_string(&config, "directory")?;
            let build = config::get_json_value_as_string(&config, "clean")?;
            clean::clean(&dir, &build)?;
        }
        Command::Dev { mode } => {
            check::check_mode(mode)?;
            test(Version::Debug, mode, config)?;
        }
        Command::Run { mode } => {
            check::check_mode(mode)?;
            test(Version::Release, mode, config)?;
        }
    }

    Ok(())
}

fn test(version: Version, mode: &String, config: serde_json::Value) -> Result<(), ()> {
    let (server_args, base_url) = config::parse_server_args(&config)?;
    let test_items = config::get_json_value(&config, "items")?;

    let wait_seconds = config::get_json_value_as_u64(&test_items, "wait_seconds")?;

    let mut dir = String::new();
    let mut bin = String::new();

    if let Version::Release = version {
        dir = config::get_json_value_as_string(&config, "directory")?;
        let build = config::get_json_value_as_string(&config, "build")?;
        build::build_server(&dir, &build)?;
        bin = config::get_json_value_as_string(&config, "bin")?;
    }

    let http_result = Some(run::http(
        &dir,
        &bin,
        &server_args,
        &base_url,
        mode,
        test_items,
        wait_seconds,
        &version,
    )?);

    let mut proxy_result = None;
    let mut pipe_result = None;
    let mut perf_result = None;

    if mode.as_str() == "advanced" {
        let items = config::get_json_value(&config, "items")?;
        let mode_items = config::get_json_value(&items, "advanced")?;
        let pipe_items = config::get_json_value(&mode_items, "pipelining")?;
        pipe_result = Some(run::pipelining(
            &dir,
            &bin,
            &server_args,
            &base_url,
            pipe_items,
            wait_seconds,
            &version,
        )?);
        let proxy_items = config::get_json_value(&mode_items, "proxy")?;
        proxy_result = Some(run::proxy(
            &dir,
            &bin,
            &server_args,
            &base_url,
            proxy_items,
            wait_seconds,
            &version,
        )?);

        let perf_items = config::get_json_value(&mode_items, "performance")?;
        perf_result = Some(run::performance(
            &dir,
            &bin,
            &server_args,
            &base_url,
            perf_items,
            wait_seconds,
            &version,
        )?);
    }

    print_results(http_result, pipe_result, proxy_result, perf_result);

    Ok(())
}

fn print_results(
    http_result: Option<(usize, usize)>,
    pipe_result: Option<(usize, usize)>,
    proxy_result: Option<(usize, usize)>,
    perf_result: Option<Vec<(usize, usize, f64, f64)>>,
) {
    info!("-------TESTER RESULTS------");
    match http_result {
        Some((all, passes)) => {
            let message = format!("HTTP test items: all {}, passes {}", all, passes);
            if all == passes {
                info!("{}", message);
            } else {
                warn!("{}", message);
            }
        }
        None => {
            warn!("HTTP not test...");
        }
    }

    match pipe_result {
        Some((all, passes)) => {
            let message = format!("Pipelining test items: all {}, passes {}", all, passes);
            if all == passes {
                info!("{}", message);
            } else {
                warn!("{}", message);
            }
        }
        None => {
            warn!("Pipelining not test...");
        }
    }

    match proxy_result {
        Some((all, passes)) => {
            let message = format!("Proxy test items: all {}, passes {}", all, passes);
            if all == passes {
                info!("{}", message);
            } else {
                warn!("{}", message);
            }
        }
        None => {
            warn!("Proxy not test...");
        }
    }

    match perf_result {
        Some(results) => {
            let len = results.len();
            info!("Perfermance test {} times.", len);
            for i in 0..len {
                let result = results.get(i).unwrap();
                info!(
                    "No.{}: requests {}, concurrency {}, reqs/s {}, time/req {}",
                    i + 1,
                    result.0,
                    result.1,
                    result.2,
                    result.3
                );
            }
        }
        None => {
            warn!("Performance not test...")
        }
    }

    info!("-------TESTER RESULTS------");
}

fn main() {
    if let Err(_) = run_cmd() {
        error!("Tester exited with errors.");
        process::exit(-1);
    }
}
