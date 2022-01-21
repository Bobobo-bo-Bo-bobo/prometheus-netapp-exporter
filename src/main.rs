#[macro_use]
extern crate simple_error;

mod aggregates;
mod config;
mod constants;
mod exporter;
mod http;
mod logging;
mod register;
mod storage_metrics;
mod usage;

use getopts::Options;
use log::error;
use std::{env, process};
use warp::Filter;

#[tokio::main]
async fn main() {
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

    println!("{:?}", config);
    println!("{}", listen_address);

    let socketaddr = match http::socketaddr_from_listen(listen_address) {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    };

    let prometheus_route = warp::path(constants::METRICS_PATH)
        .and(exporter::pass_configuration(config.clone()))
        .and(warp::get())
        .map(exporter::serve_metrics);

    let root_route = warp::path::end()
        .and(warp::get())
        .map(move || warp::reply::html(constants::ROOT_HTML.to_string()));

    let route = root_route.or(prometheus_route);

    // XXX: async rust with tokio might provide a better solution enable graceful shutdown
    //      e.g. https://docs.rs/warp/latest/warp/struct.Server.html#method.bind_with_graceful_shutdown
    warp::serve(route).run(socketaddr).await;
}
