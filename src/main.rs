#[macro_use]
extern crate simple_error;

mod aggregates;
mod chassis;
mod config;
mod constants;
mod exporter;
mod http;
mod logging;
mod quotas;
mod register;
mod storage_metrics;
mod usage;
mod volumes;

use getopts::Options;
use log::error;
use std::{env, process};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut options = Options::new();
    let mut log_level = log::LevelFilter::Info;

    options.optflag("D", "debug", "Enable debug log");
    options.optflag("V", "version", "Show version");
    options.optopt("c", "config", "Configuration file", "<config_file>");
    options.optflag("h", "help", "Show help text");
    options.optopt("l", "listen", "Listen address", "<address>");
    options.optflag("q", "quiet", "Quiet operation");

    let opts = match options.parse(&argv[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Can't parse command line arguments: {}", e);
            println!();
            usage::show_usage();
            process::exit(1);
        }
    };

    if opts.opt_present("h") {
        usage::show_usage();
        process::exit(0);
    }

    if opts.opt_present("V") {
        usage::show_version();
        process::exit(0);
    }

    if opts.opt_present("D") {
        log_level = log::LevelFilter::Debug;
    }

    if opts.opt_present("q") {
        log_level = log::LevelFilter::Warn;
    }

    let config_file = match opts.opt_str("c") {
        Some(v) => v,
        None => {
            eprintln!("Error: Configuration file is mandatory");
            println!();
            usage::show_usage();
            process::exit(1);
        }
    };

    let listen_address = opts
        .opt_str("l")
        .unwrap_or_else(|| constants::DEFAULT_PROMETHEUS_ADDRESS.to_string());

    let config = match config::parse_config_file(&config_file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Configuration parsing failed: {}", e);
            process::exit(1);
        }
    };

    match logging::init(log_level) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: Can't initialise logging: {}", e);
            process::exit(1);
        }
    };

    exporter::register_aggregate_metrics();
    exporter::register_quota_metrics();
    exporter::register_volume_metrics();
    exporter::register_chassis_metrics();

    if let Err(e) = http::server(config, &listen_address) {
        error!("Cen't start HTTP server: {}", e);
        process::exit(1);
    };
}
