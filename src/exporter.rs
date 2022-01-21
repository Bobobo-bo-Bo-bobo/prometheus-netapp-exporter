use crate::config;

use lazy_static::lazy_static;
use log::error;
use prometheus::Registry;
use std::convert::Infallible;
use warp::Filter;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    // Aggregate data
}

pub fn pass_configuration(
    cfg: config::Configuration,
) -> impl Filter<Extract = (config::Configuration,), Error = Infallible> + Clone {
    warp::any().map(move || cfg.clone())
}

pub fn serve_metrics(cfg: config::Configuration) -> String {
    // Scrape filers

    /*
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = String::new();

    if let Err(e) = encoder.encode_utf8(&REGISTRY.gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    }

    if let Err(e) = encoder.encode_utf8(&prometheus::gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    };

    buffer
    */
    format!("{:?}", cfg)
}
