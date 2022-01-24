use crate::aggregates;
use crate::config;
use crate::constants;
use crate::http;

use lazy_static::lazy_static;
use log::{error, info};
use prometheus::{GaugeVec, IntGaugeVec, Opts, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    // Aggregate data

    pub static ref AGGREGATE_FOOTPRINT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_FOOTPRINT_NAME, constants::METRIC_AGGR_FOOTPRINT_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();

    pub static ref AGGREGATE_BLOCK_STORAGE_SIZE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_SIZE_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_SIZE_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_USED_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_USED_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_AVAILABLE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_AVAILABLE_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_AVAILABLE_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_FULL_THRESHOLD: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_FULL_THRESHOLD_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_FULL_THRESHOLD_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();

    pub static ref AGGREGATE_BLOCK_STORAGE_EFFICENCY_LOGICAL_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_LOGICAL_USED_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_LOGICAL_USED_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_EFFICENCY_SAVINGS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_SAVINGS_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_SAVINGS_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_EFFICENCY_RATIO: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_RATIO_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_RATIO_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();

    pub static ref AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_LOGICAL_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_LOGICAL_USED_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_LOGICAL_USED_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_SAVINGS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_SAVINGS_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_SAVINGS_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_RATIO: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_RATIO_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_RATIO_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();

    pub static ref AGGREGATE_CLOUD_STORAGE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_CLOUD_STORAGE_USED_NAME, constants::METRIC_AGGR_CLOUD_STORAGE_USED_HELP),
        &["filer", "home_node", "aggregate"]
    ).unwrap();

    pub static ref AGGREGATE_BLOCK_STORAGE_PLEXES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_PLEXES_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_PLEXES_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();

    pub static ref AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_ENABLED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_ENABLED_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_ENABLED_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_SIZE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_SIZE_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_SIZE_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_USED_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_USED_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_DISK_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_DISK_USED_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_DISK_USED_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();

    pub static ref AGGREGATE_BLOCK_STORAGE_PRIMARY_DISKS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_PRIMARY_DISK_COUNT_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_PRIMARY_DISK_COUNT_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_PRIMARY_RAID_SIZE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_PRIMARY_RAID_SIZE_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_PRIMARY_RAID_SIZE_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();

    pub static ref AGGREGATE_BLOCK_STORAGE_MIRROR_ENABLED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_MIRROR_ENABLED_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_MIRROR_ENABLED_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_BLOCK_STORAGE_MIRROR_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_BLOCK_STORAGE_MIRROR_STATE_NAME, constants::METRIC_AGGR_BLOCK_STORAGE_MIRROR_STATE_HELP),
        &["filer", "home_node", "aggregate", "state"],
    ).unwrap();

    pub static ref AGGREGATE_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_STATE_NAME, constants::METRIC_AGGR_STATE_HELP),
        &["filer", "home_node", "aggregate", "state"],
    ).unwrap();

    pub static ref AGGREGATE_METRIC_THROUGHPUT_READ: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_THROUGHPUT_READ_NAME, constants::METRIC_AGGR_METRIC_THROUGHPUT_READ_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_THROUGHPUT_WRITE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_THROUGHPUT_WRITE_NAME, constants::METRIC_AGGR_METRIC_THROUGHPUT_WRITE_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_THROUGHPUT_OTHER: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_THROUGHPUT_OTHER_NAME, constants::METRIC_AGGR_METRIC_THROUGHPUT_OTHER_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_THROUGHPUT_TOTAL: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_THROUGHPUT_TOTAL_NAME, constants::METRIC_AGGR_METRIC_THROUGHPUT_TOTAL_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();

    pub static ref AGGREGATE_METRIC_LATENCY_READ: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_LATENCY_READ_NAME, constants::METRIC_AGGR_METRIC_LATENCY_READ_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_LATENCY_WRITE: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_LATENCY_WRITE_NAME, constants::METRIC_AGGR_METRIC_LATENCY_WRITE_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_LATENCY_OTHER: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_LATENCY_OTHER_NAME, constants::METRIC_AGGR_METRIC_LATENCY_OTHER_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_LATENCY_TOTAL: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_LATENCY_TOTAL_NAME, constants::METRIC_AGGR_METRIC_LATENCY_TOTAL_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();

    pub static ref AGGREGATE_METRIC_IOPS_READ: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_IOPS_READ_NAME, constants::METRIC_AGGR_METRIC_IOPS_READ_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_IOPS_WRITE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_IOPS_WRITE_NAME, constants::METRIC_AGGR_METRIC_IOPS_WRITE_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_IOPS_OTHER: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_IOPS_OTHER_NAME, constants::METRIC_AGGR_METRIC_IOPS_OTHER_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_IOPS_TOTAL: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_IOPS_TOTAL_NAME, constants::METRIC_AGGR_METRIC_IOPS_TOTAL_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
    pub static ref AGGREGATE_METRIC_SAMPLE_DURATION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_AGGR_METRIC_SAMPLE_DURATION_NAME, constants::METRIC_AGGR_METRIC_SAMPLE_DURATION_HELP),
        &["filer", "home_node", "aggregate"],
    ).unwrap();
}

fn update_metrics(filer: &config::NetAppConfiguration, client: &mut reqwest::blocking::Client) {
    if filer.targets_mask & constants::TARGET_AGGREGATES == constants::TARGET_AGGREGATES {
        info!("Requesting aggregate information from {}", filer.name);
        if let Err(e) = aggregates::update_aggregates(filer, client) {
            error!(
                "Unable to update aggregate statistics for {} - {}",
                filer.name, e
            );
        }
    }
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

    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_SIZE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_USED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_AVAILABLE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_FULL_THRESHOLD.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(
            AGGREGATE_BLOCK_STORAGE_EFFICENCY_LOGICAL_USED.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_EFFICENCY_SAVINGS.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_EFFICENCY_RATIO.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(
            AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_LOGICAL_USED.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(
            AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_SAVINGS.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(
            AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_RATIO.clone(),
        ))
        .unwrap();

    REGISTRY
        .register(Box::new(AGGREGATE_CLOUD_STORAGE.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_PLEXES.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(
            AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_ENABLED.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_SIZE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_USED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(
            AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_DISK_USED.clone(),
        ))
        .unwrap();

    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_PRIMARY_DISKS.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_PRIMARY_RAID_SIZE.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_MIRROR_ENABLED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_BLOCK_STORAGE_MIRROR_STATE.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(AGGREGATE_STATE.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_THROUGHPUT_READ.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_THROUGHPUT_WRITE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_THROUGHPUT_OTHER.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_THROUGHPUT_TOTAL.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_LATENCY_READ.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_LATENCY_WRITE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_LATENCY_OTHER.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_LATENCY_TOTAL.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_IOPS_READ.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_IOPS_WRITE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_IOPS_OTHER.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_IOPS_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(AGGREGATE_METRIC_SAMPLE_DURATION.clone()))
        .unwrap();
}
