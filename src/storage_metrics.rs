use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct StorageMetric {
    pub duration: String,
    pub iops: StorageMetricData,
    pub latency: StorageMetricData,
    pub status: String,
    pub throughput: StorageMetricData,
    pub timestamp: String,
}

#[derive(Deserialize, Clone)]
pub struct StorageMetricData {
    pub other: u64,
    pub read: u64,
    pub total: u64,
    pub write: u64,
}

#[derive(Deserialize, Clone)]
pub struct StorageStatistics {
    pub timestamp: String,
    pub status: String,
    pub latency_raw: StorageMetricData,
    pub iops_raw: StorageMetricData,
    pub throughput_raw: StorageMetricData,
}
