use crate::config;
use crate::constants;
use crate::exporter;
use crate::http;
use crate::storage_metrics;

use log::{debug, error, warn};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateList {
    pub records: Vec<Aggregate>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Aggregate {
    pub block_storage: AggregateBlockStorage,
    pub home_node: NodeInfo,
    // Requires at least OnTap 9.7
    pub metric: Option<storage_metrics::StorageMetric>,
    pub name: String,
    pub node: NodeInfo,
    pub snaplock_type: String,
    pub space: AggregateSpace,
    pub state: String,
    // Requires at least OnTap 9.7
    pub statistics: Option<storage_metrics::StorageStatistics>,
    pub uuid: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateBlockStorage {
    pub hybrid_cache: AggregateBlockStorageHybridCache,
    pub mirror: AggregateBlockStorageMirror,
    pub plexes: Vec<AggregateBlockStoragePlex>,
    pub primary: AggregateBlockStoragePrimary,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateBlockStorageMirror {
    pub enabled: bool,
    pub state: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateBlockStorageHybridCache {
    pub raid_type: Option<String>,
    pub used: Option<i64>,
    pub disk_count: Option<i64>,
    pub size: Option<i64>,
    pub enabled: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateBlockStoragePrimary {
    pub raid_type: String,
    pub disk_class: String,
    pub checksum_style: String,
    pub disk_type: String,
    pub disk_count: i64,
    pub raid_size: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateBlockStoragePlex {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NodeInfo {
    pub name: String,
    pub uuid: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateSpace {
    pub block_storage: AggregateSpaceBlockStorage,
    pub cloud_storage: AggregateSpaceCloudStorage,
    pub efficiency: AggregateSpaceStorageEfficiency,
    pub efficiency_without_snapshots: AggregateSpaceStorageEfficiency,
    pub footprint: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateSpaceCloudStorage {
    pub used: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateSpaceBlockStorage {
    pub available: i64,
    pub full_threshold_percent: i64,
    pub size: i64,
    pub used: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateSpaceStorageEfficiency {
    pub logical_used: i64,
    pub ratio: f64,
    pub savings: i64,
}

pub fn update_aggregates(
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

    let aggrs: AggregateList = match serde_json::from_str(&raw_aggrs) {
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
        exporter::AGGREGATE_FOOTPRINT
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.footprint);

        debug!(
            "Updating metrics for aggregate block_storage size: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.block_storage.size
        );
        exporter::AGGREGATE_BLOCK_STORAGE_SIZE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.block_storage.size);

        debug!(
            "Updating metrics for aggregate block_storage used: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.block_storage.used
        );
        exporter::AGGREGATE_BLOCK_STORAGE_USED
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.block_storage.used);

        debug!(
            "Updating metrics for aggregate block_storage available: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.block_storage.available
        );
        exporter::AGGREGATE_BLOCK_STORAGE_AVAILABLE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.block_storage.available);

        debug!(
            "Updating metrics for aggregate block_storage full_threshold: {} {} {} -> {}",
            filer.name,
            aggr.home_node.name,
            aggr.name,
            aggr.space.block_storage.full_threshold_percent
        );
        exporter::AGGREGATE_BLOCK_STORAGE_FULL_THRESHOLD
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.block_storage.full_threshold_percent);

        debug!(
            "Updating metrics for aggregate efficiency logical_used: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.efficiency.logical_used
        );
        exporter::AGGREGATE_BLOCK_STORAGE_EFFICENCY_LOGICAL_USED
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency.logical_used);

        debug!(
            "Updating metrics for aggregate efficiency savings: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.efficiency.savings
        );
        exporter::AGGREGATE_BLOCK_STORAGE_EFFICENCY_SAVINGS
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency.savings);

        debug!(
            "Updating metrics for aggregate efficiency ratio: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.efficiency.ratio
        );
        exporter::AGGREGATE_BLOCK_STORAGE_EFFICENCY_RATIO
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency.ratio);

        debug!(
                "Updating metrics for aggregate efficiency_without_snapshots logical_used: {} {} {} -> {}",
                filer.name,
                aggr.home_node.name,
                aggr.name,
                aggr.space.efficiency_without_snapshots.logical_used
            );
        exporter::AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_LOGICAL_USED
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency_without_snapshots.logical_used);

        debug!(
            "Updating metrics for aggregate efficiency_without_snapshots savings: {} {} {} -> {}",
            filer.name,
            aggr.home_node.name,
            aggr.name,
            aggr.space.efficiency_without_snapshots.savings
        );
        exporter::AGGREGATE_BLOCK_STORAGE_EFFICENCY_WO_SNAPSHOTS_SAVINGS
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency_without_snapshots.savings);

        debug!(
            "Updating metrics for aggregate efficiency_without_snapshots ratio: {} {} {} -> {}",
            filer.name,
            aggr.home_node.name,
            aggr.name,
            aggr.space.efficiency_without_snapshots.ratio
        );
        exporter::AGGREGATE_BLOCK_STORAGE_EFFICENCY_RATIO
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.efficiency_without_snapshots.ratio);

        debug!(
            "Updating metrics for aggregate cloud_storage used: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.space.cloud_storage.used
        );
        exporter::AGGREGATE_CLOUD_STORAGE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.space.cloud_storage.used);

        debug!(
            "Updating metrics for aggregate plexes count: {} {} {} -> {}",
            filer.name,
            aggr.home_node.name,
            aggr.name,
            aggr.block_storage.plexes.len()
        );
        exporter::AGGREGATE_BLOCK_STORAGE_PLEXES
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.block_storage.plexes.len() as i64);

        debug!(
            "Updating metrics for aggregate hybrid_cache enabled: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.block_storage.hybrid_cache.enabled
        );
        if aggr.block_storage.hybrid_cache.enabled {
            exporter::AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_ENABLED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(1);
        } else {
            exporter::AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_ENABLED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(0);
        }

        if let Some(v) = aggr.block_storage.hybrid_cache.disk_count {
            debug!(
                "Updating metrics for aggregate hybrid_cache disk_used: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v
            );
            exporter::AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_DISK_USED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v);
        }
        if let Some(v) = aggr.block_storage.hybrid_cache.size {
            debug!(
                "Updating metrics for aggregate hybrid_cache size: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v
            );
            exporter::AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_SIZE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v);
        }
        if let Some(v) = aggr.block_storage.hybrid_cache.used {
            debug!(
                "Updating metrics for aggregate hybrid_cache used: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v
            );
            exporter::AGGREGATE_BLOCK_STORAGE_HYBRID_CACHE_USED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v);
        }

        debug!(
            "Updating metrics for aggregate primary disk_count: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.block_storage.primary.disk_count
        );
        exporter::AGGREGATE_BLOCK_STORAGE_PRIMARY_DISKS
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.block_storage.primary.disk_count);

        debug!(
            "Updating metrics for aggregate primary raid_size: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.block_storage.primary.raid_size
        );
        exporter::AGGREGATE_BLOCK_STORAGE_PRIMARY_RAID_SIZE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
            .set(aggr.block_storage.primary.raid_size);

        debug!(
            "Updating metrics for aggregate mirror enabled: {} {} {} -> {}",
            filer.name, aggr.home_node.name, aggr.name, aggr.block_storage.mirror.enabled
        );
        if aggr.block_storage.mirror.enabled {
            exporter::AGGREGATE_BLOCK_STORAGE_MIRROR_ENABLED
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(1);
        } else {
            exporter::AGGREGATE_BLOCK_STORAGE_MIRROR_ENABLED
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
            exporter::AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "unmirrored"])
                .set(unmirrored);
            exporter::AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "normal"])
                .set(normal);
            exporter::AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "degraded"])
                .set(degraded);
            exporter::AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
                .with_label_values(&[
                    &filer.name,
                    &aggr.home_node.name,
                    &aggr.name,
                    "resynchronizing",
                ])
                .set(resynchronizing);
            exporter::AGGREGATE_BLOCK_STORAGE_MIRROR_STATE
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

        exporter::AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "online"])
            .set(online);
        exporter::AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "onlining"])
            .set(onlining);
        exporter::AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "offline"])
            .set(offline);
        exporter::AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "offlining"])
            .set(offlining);
        exporter::AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "relocating"])
            .set(relocating);
        exporter::AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "unmounted"])
            .set(unmounted);
        exporter::AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "restricted"])
            .set(restricted);
        exporter::AGGREGATE_STATE
            .with_label_values(&[
                &filer.name,
                &aggr.home_node.name,
                &aggr.name,
                "inconsistent",
            ])
            .set(inconsistent);
        exporter::AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "failed"])
            .set(failed);
        exporter::AGGREGATE_STATE
            .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name, "unknown"])
            .set(unknown);

        if let Some(v) = aggr.metric {
            if v.status != "ok" {
                warn!("Skipping metrics from aggregate {} on {} because metric state was reported as \"{}\" instead of \"ok\"", aggr.name, filer.name, v.status);
                continue;
            };

            debug!(
                "Updating metrics for aggregate metric duration: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.duration
            );
            let duration: i64 = match v.duration.as_str() {
                "PT15S" => 15,
                "PT1D" => 86400,
                "PT2H" => 7200,
                "PT30M" => 1800,
                "PT4M" => 240,
                "PT5M" => 300,
                _ => {
                    error!(
                        "Invalid or unsupported sample duration {} for aggregate {} on {}",
                        v.duration, aggr.name, filer.name
                    );
                    continue;
                }
            };
            exporter::AGGREGATE_METRIC_SAMPLE_DURATION
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(duration);

            debug!(
                "Updating metrics for aggregate metric throughput read: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.throughput.read
            );
            exporter::AGGREGATE_METRIC_THROUGHPUT_READ
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.throughput.read);

            debug!(
                "Updating metrics for aggregate metric throughput write: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.throughput.write
            );
            exporter::AGGREGATE_METRIC_THROUGHPUT_WRITE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.throughput.write);

            debug!(
                "Updating metrics for aggregate metric throughput other: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.throughput.other
            );
            exporter::AGGREGATE_METRIC_THROUGHPUT_OTHER
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.throughput.other);

            debug!(
                "Updating metrics for aggregate metric throughput total: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.throughput.total
            );
            exporter::AGGREGATE_METRIC_THROUGHPUT_TOTAL
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.throughput.total);

            debug!(
                "Updating metrics for aggregate metric latency read: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.latency.read
            );
            exporter::AGGREGATE_METRIC_LATENCY_READ
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.latency.read as f64 / 1e+06);

            debug!(
                "Updating metrics for aggregate metric latency write: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.latency.write
            );
            exporter::AGGREGATE_METRIC_LATENCY_WRITE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.latency.write as f64 / 1e+06);

            debug!(
                "Updating metrics for aggregate metric latency other: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.latency.other
            );
            exporter::AGGREGATE_METRIC_LATENCY_OTHER
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.latency.other as f64 / 1e+06);

            debug!(
                "Updating metrics for aggregate metric latency total: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.latency.total
            );
            exporter::AGGREGATE_METRIC_LATENCY_TOTAL
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.latency.total as f64 / 1e+06);

            debug!(
                "Updating metrics for aggregate metric iops read: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.iops.read
            );
            exporter::AGGREGATE_METRIC_IOPS_READ
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.iops.read);

            debug!(
                "Updating metrics for aggregate metric iops write: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.iops.write
            );
            exporter::AGGREGATE_METRIC_IOPS_WRITE
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.iops.write);

            debug!(
                "Updating metrics for aggregate metric iops other: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.iops.other
            );
            exporter::AGGREGATE_METRIC_IOPS_OTHER
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.iops.other);

            debug!(
                "Updating metrics for aggregate metric iops total: {} {} {} -> {}",
                filer.name, aggr.home_node.name, aggr.name, v.iops.total
            );
            exporter::AGGREGATE_METRIC_IOPS_TOTAL
                .with_label_values(&[&filer.name, &aggr.home_node.name, &aggr.name])
                .set(v.iops.total);
        }
    }

    Ok(())
}
