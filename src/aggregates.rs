use crate::storage_metrics;

use serde::Deserialize;

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
