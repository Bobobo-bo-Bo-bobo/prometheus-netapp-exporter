use crate::aggregates;
use crate::config;
use crate::constants;
use crate::http;

use lazy_static::lazy_static;
use log::{debug, error, info};
use prometheus::{GaugeVec, IntGaugeVec, Opts, Registry};
use std::error::Error;

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

        debug!(
            "Updating metrics for aggregate block_storage size: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.block_storage.size
        );
        AGGREGATE_BLOCK_STORAGE_SIZE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.block_storage.size);

        debug!(
            "Updating metrics for aggregate block_storage used: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.block_storage.used
        );
        AGGREGATE_BLOCK_STORAGE_USED
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.block_storage.used);

        debug!(
            "Updating metrics for aggregate block_storage available: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.block_storage.available
        );
        AGGREGATE_BLOCK_STORAGE_AVAILABLE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.block_storage.available);

        debug!(
            "Updating metrics for aggregate block_storage full_threshold: {} {} {} -> {}",
            filer.name,
            aggr.home_node.name,
            aggr.name,
            aggr.space.block_storage.full_threshold_percent
        );
        AGGREGATE_BLOCK_STORAGE_FULL_THRESHOLD
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.block_storage.full_threshold_percent);

        debug!(
            "Updating metrics for aggregate efficiency logical_used: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.efficiency.logical_used
        );
        AGGREGATE_BLOCK_STORAGE_EFFICENCY_LOGICAL_USED
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency.logical_used);

        debug!(
            "Updating metrics for aggregate efficiency savings: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.efficiency.savings
        );
        AGGREGATE_BLOCK_STORAGE_EFFICENCY_SAVINGS
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency.savings);

        debug!(
            "Updating metrics for aggregate efficiency ratio: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.efficiency.ratio
        );
        AGGREGATE_BLOCK_STORAGE_EFFICENCY_RATIO
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency.ratio);

        debug!(
                "Updating metrics for aggregate efficiency_without_snapshots logical_used: {} {} {} -> {}",
                filer.name,
                aggr.home_node.name,
                aggr.name,
                aggr.space.efficiency_without_snapshots.logical_used
            );
        AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_LOGICAL_USED
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency_without_snapshots.logical_used);

        debug!(
            "Updating metrics for aggregate efficiency_without_snapshots savings: {} {} {} -> {}",
            filer.name,
            aggr.home_node.name,
            aggr.name,
            aggr.space.efficiency_without_snapshots.savings
        );
        AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_SAVINGS
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency_without_snapshots.savings);

        debug!(
            "Updating metrics for aggregate efficiency_without_snapshots ratio: {} {} {} -> {}",
            filer.name,
            aggr.home_node.name,
            aggr.name,
            aggr.space.efficiency_without_snapshots.ratio
        );
        AGGREGATE_BLOCK_STORAGE_EFFICENCY_RATIO
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency_without_snapshots.ratio);

        debug!(
            "Updating metrics for aggregate cloud_storage used: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.cloud_storage.used
        );
        AGGREGATE_CLOUD_STORAGE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.cloud_storage.used);

        debug!(
            "Updating metrics for aggregate plexes count: {} {} {} -> {}",
            filer.name,
            aggr.home_node.name,
            aggr.name,
            aggr.block_storage.plexes.len()
        );
        AGGREGATE_BLOCK_STORAGE_PLEXES
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.block_storage.plexes.len() as i64);

        debug!(
            "Updating metrics for aggregate hybrid_cache enabled: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.block_storage.hybrid_cache.enabled
        );
        if aggr.block_storage.hybrid_cache.enabled {
            AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_ENABLED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(1);
        } else {
            AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_ENABLED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(0);
        }

        if let Some(v) = aggr.block_storage.hybrid_cache.disk_count {
            debug!(
                "Updating metrics for aggregate hybrid_cache disk_used: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v
            );
            AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_DISK_USED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v);
        }
        if let Some(v) = aggr.block_storage.hybrid_cache.size {
            debug!(
                "Updating metrics for aggregate hybrid_cache size: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v
            );
            AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_SIZE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v);
        }
        if let Some(v) = aggr.block_storage.hybrid_cache.used {
            debug!(
                "Updating metrics for aggregate hybrid_cache used: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v
            );
            AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_USED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v);
        }

        debug!(
            "Updating metrics for aggregate primary disk_count: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.block_storage.primary.disk_count
        );
        AGGREGATE_BLOCK_STORAGE_PRIMARY_DISKS
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.block_storage.primary.disk_count);

        debug!(
            "Updating metrics for aggregate primary raid_size: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.block_storage.primary.raid_size
        );
        AGGREGATE_BLOCK_STORAGE_PRIMARY_RAID_SIZE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.block_storage.primary.raid_size);

        debug!(
            "Updating metrics for aggregate mirror enabled: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.block_storage.mirror.enabled
        );
        if aggr.block_storage.mirror.enabled {
            AGGREGATE_BLOCK_STORAGE_MIRROR_ENABLED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(1);
        } else {
            AGGREGATE_BLOCK_STORAGE_MIRROR_ENABLED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(0);
        }
        if let Some(v) = aggr.block_storage.mirror.state {
            debug!(
                "Updating metrics for aggregate mirror state: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v
            );
            let mut unmirrored: i64 = 0;
            let mut normal: i64 = 0;
            let mut degraded: i64 = 0;
            let mut resynchronizing: i64 = 0;
            let mut failed: i64 = 0;

            match v.as_str() {
                "unmirrored" => {
                    unmirrored = 1;
                }
                "normal" => {
                    normal = 1;
                }
                "degraded" => {
                    degraded = 1;
                }
                "resynchronizing" => {
                    resynchronizing = 1;
                }
                "failed" => {
                    failed = 1;
                }
                _ => {
                    error!(
                        "Unknown state for SyncMirror on aggregate {} of {}: {}",
                        aggr.name, filer.name, v
                    );
                    continue;
                }
            };
            AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "unmirrored"])
                .set(unmirrored);
            AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "normal"])
                .set(normal);
            AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "degraded"])
                .set(degraded);
            AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
                .with_label_values(&[
                    &filer.name,
                    &aggr.home_node.name,
                    &aggr.name,
                    "resynchronizing",
                ])
                .set(resynchronizing);
            AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "failed"])
                .set(failed);
        }
        debug!(
            "Updating metrics for aggregate state: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.state
        );
        let mut online: i64 = 0;
        let mut onlining: i64 = 0;
        let mut offline: i64 = 0;
        let mut offlining: i64 = 0;
        let mut relocating: i64 = 0;
        let mut unmounted: i64 = 0;
        let mut restricted: i64 = 0;
        let mut inconsistent: i64 = 0;
        let mut failed: i64 = 0;
        let mut unknown: i64 = 0;

        match aggr.state.as_str() {
            "online" => {
                online = 1;
            }
            "onlining" => {
                onlining = 1;
            }
            "offline" => {
                offline = 1;
            }
            "offlining" => {
                offlining = 1;
            }
            "relocating" => {
                relocating = 1;
            }
            "unmounted" => {
                unmounted = 1;
            }
            "restricted" => {
                restricted = 1;
            }
            "inconsistent" => {
                inconsistent = 1;
            }
            "failed" => {
                failed = 1;
            }
            "unknown" => {
                unknown = 1;
            }
            _ => {
                error!(
                    "Unknown aggregate state {} for aggregate {} on {}",
                    aggr.state, aggr.name, filer.name
                );
                continue;
            }
        };

        AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "online"])
            .set(online);
        AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "onlining"])
            .set(onlining);
        AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "offline"])
            .set(offline);
        AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "offlining"])
            .set(offlining);
        AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "relocating"])
            .set(relocating);
        AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "unmounted"])
            .set(unmounted);
        AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "restricted"])
            .set(restricted);
        AGGREGATE_STATE
            .with_label_values(&[
                &filer.name,
                &aggr.home_node.name,
                &aggr.name,
                "inconsistent",
            ])
            .set(inconsistent);
        AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "failed"])
            .set(failed);
        AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "unknown"])
            .set(unknown);
    }
    Ok(())
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
}
