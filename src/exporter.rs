use crate::config;
use crate::constants;
use crate::http;

use lazy_static::lazy_static;
use log::error;
use prometheus::Registry;
use std::error::Error;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    // Aggregate data
}

fn update_metrics(
    filer: config::NetAppConfiguration,
    client: &mut reqwest::Client,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn serve_metrics(config: config::Configuration) -> String {
    let cfg = config.clone();
    // Scrape filers
    let filers = cfg.filer.clone();
    for flr in filers {
        let mut client = match flr.http_client {
            Some(v) => v,
            None => panic!(
                "BUG: exporter.rs: HTTP client structure is None for {}",
                flr.name
            ),
        };
        let url = format!(
            "https://{}{}?fields=**",
            flr.address,
            constants::API_AGGREGATES
        );
        let aggrs = match http::get(&mut client, &url, &flr.user, &flr.password) {
            Ok(v) => v,
            Err(e) => {
                error!("Request for aggregates on {} failed - {}", flr.name, e);
                continue;
            }
        };
    }
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
    "...".to_string()
}
