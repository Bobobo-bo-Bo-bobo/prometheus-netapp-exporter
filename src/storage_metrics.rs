use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct StorageMetric {
    pub duration: String,
    pub iops: StorageMetricData,
    pub latency: StorageMetricData,
    pub status: String,
    pub throughput: StorageMetricData,
    pub timestamp: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct StorageMetricData {
    pub other: i64,
    pub read: i64,
    pub total: i64,
    pub write: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct StorageStatistics {
    pub timestamp: String,
    pub status: String,
    pub latency_raw: StorageMetricData,
    pub iops_raw: StorageMetricData,
    pub throughput_raw: StorageMetricData,
}
