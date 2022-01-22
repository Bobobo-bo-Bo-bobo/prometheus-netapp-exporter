use crate::storage_metrics;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateList {
    pub records: Vec<Aggregate>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Aggregate {
    pub home_node: NodeInfo,
    // Requires at least OnTap 9.7
    pub metric: Option<storage_metrics::StorageMetric>,
    pub name: String,
    pub node: NodeInfo,
    pub snaplock_type: String,
    pub space: AggregateSpace,
    // Requires at least OnTap 9.7
    pub statistics: Option<storage_metrics::StorageStatistics>,
    pub uuid: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NodeInfo {
    pub name: String,
    pub uuid: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateSpace {
    pub block_storage: AggregateBlockStorage,
    pub cloud_storage: AggregateSpaceCloudStorage,
    pub efficiency: AggregateSpaceStorageEfficiency,
    pub efficiency_without_snapshots: AggregateSpaceStorageEfficiency,
    pub footprint: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateSpaceCloudStorage {
    pub used: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateBlockStorage {
    pub available: u64,
    pub full_threshold_percent: u8,
    pub size: u64,
    pub used: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateSpaceStorageEfficiency {
    pub logical_used: u64,
    pub ratio: f64,
    pub savings: u64,
}
