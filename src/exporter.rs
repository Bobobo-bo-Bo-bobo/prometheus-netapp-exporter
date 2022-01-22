use crate::aggregates;
use crate::config;
use crate::constants;
use crate::http;

use lazy_static::lazy_static;
use log::{debug, error, info};
use prometheus::{IntGaugeVec, Opts, Registry};
use std::error::Error;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    // Aggregate data

    pub static ref AGGREGATE_FOOTPRINT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_FOOTPRINT_NAME, constants::METRIC_AGGR_FOOTPRINT_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();
}

fn update_metrics(filer: &config::NetAppConfiguration, client: &mut reqwest::blocking::Client) {
    if filer.targets_mask & constants::TARGET_AGGREGATES == constants::TARGET_AGGREGATES {
        info!("Requesting aggregate information from {}", filer.name);
        if let Err(e) = update_aggregates(filer, client) {
            error!(
                "Unable to update aggregate statistics for {} - {}",
                filer.name, e
            );
        }
    }
}

fn update_aggregates(
    filer: &config::NetAppConfiguration,
    client: &mut reqwest::blocking::Client,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://{}{}?fields=**",
        filer.address,
        constants::API_AGGREGATES
    );
    let raw_aggrs = match http::get(client, &url, &filer.user, &filer.password) {
        Ok(v) => v,
        Err(e) => {
            bail!("Request for aggregates on {} failed - {}", filer.name, e);
        }
    };

    let aggrs: aggregates::AggregateList = match serde_json::from_str(&raw_aggrs) {
        Ok(v) => v,
        Err(e) => bail!(
            "Can't decode response for aggregate information from {} as JSON - {}",
            filer.name,
            e
        ),
    };

    for aggr in aggrs.records {
        debug!(
            "Updating metrics for aggregate footprint: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.footprint
        );
        AGGREGATE_FOOTPRINT
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.footprint);
    }
    Ok(())
}

pub fn serve_metrics(cfg: &config::Configuration) -> String {
    let filers = &cfg.filer;

    for flr in filers {
        let insecure_ssl = flr.insecure_ssl.unwrap_or(constants::DEFAULT_INSECURE_SSL);
        let ca_file = flr.ca_cert.clone().unwrap_or_default();
        let timeout_sec = flr.timeout.unwrap_or(constants::DEFAULT_TIMEOUT);
        let mut client = match http::build_client(insecure_ssl, &ca_file, timeout_sec) {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "Skipping scrape for {} - can't build HTTP client: {}",
                    flr.name, e
                );
                continue;
            }
        };
        update_metrics(flr, &mut client);
    }
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = String::new();

    if let Err(e) = encoder.encode_utf8(&REGISTRY.gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    }

    if let Err(e) = encoder.encode_utf8(&prometheus::gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    };
    buffer
}

pub fn register_metrics() {
    REGISTRY
        .register(Box::new(AGGREGATE_FOOTPRINT.clone()))
        .unwrap();
}
