use crate::aggregates;
use crate::chassis;
use crate::cifs;
use crate::config;
use crate::constants;
use crate::ethernet;
use crate::fibrechannel;
use crate::http;
use crate::jobs;
use crate::nfs;
use crate::quotas;
use crate::volumes;

use lazy_static::lazy_static;
use log::{error, info};
use prometheus::{GaugeVec, IntCounterVec, IntGaugeVec, Opts, Registry};

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

// NOTE: macro split is required to suppress the "recursion limit reached while expanding `__lazy_static_internal!`" error
lazy_static! {
    // Volume data
    pub static ref VOLUME_FILES_MAX: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_FILES_MAXIMUM_NAME, constants::METRIC_VOL_FILES_MAXIMUM_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_FILES_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_FILES_USED_NAME, constants::METRIC_VOL_FILES_USED_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_STATE_NAME, constants::METRIC_VOL_STATE_HELP),
        &["filer", "volume", "state"]
    ).unwrap();

    pub static ref VOLUME_ERROR_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_ERROR_STATE_NAME, constants::METRIC_VOL_ERROR_STATE_HELP),
        &["filer", "volume", "error_state"]
    ).unwrap();

    pub static ref VOLUME_AUTOSIZE_MIN: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_AUTOSIZE_MINIMUM_NAME, constants::METRIC_VOL_AUTOSIZE_MINIMUM_HELP),
        &["filer", "volume"]
    ).unwrap();
    pub static ref VOLUME_AUTOSIZE_MAX: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_AUTOSIZE_MAXIMUM_NAME, constants::METRIC_VOL_AUTOSIZE_MAXIMUM_HELP),
        &["filer", "volume"]
    ).unwrap();
    pub static ref VOLUME_AUTOSIZE_MODE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_AUTOSIZE_MODE_NAME, constants::METRIC_VOL_AUTOSIZE_MODE_HELP),
        &["filer", "volume", "mode"]
    ).unwrap();
    pub static ref VOLUME_AUTOSIZE_SHRINK_THRESHOLD: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_AUTOSIZE_SHRINK_THRESHOLD_NAME, constants::METRIC_VOL_AUTOSIZE_SHRINK_THRESHOLD_HELP),
        &["filer", "volume"]
    ).unwrap();
    pub static ref VOLUME_AUTOSIZE_GROW_THRESHOLD: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_AUTOSIZE_GROW_THRESHOLD_NAME, constants::METRIC_VOL_AUTOSIZE_GROW_THRESHOLD_HELP),
        &["filer", "volume"]
    ).unwrap();
    pub static ref VOLUME_IS_OBJECT_STORE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_IS_OBJECT_STORE_NAME, constants::METRIC_VOL_IS_OBJECT_STORE_HELP),
        &["filer", "volume"]
    ).unwrap();
    pub static ref VOLUME_NUMBER_OF_AGGREGATES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_NUMBER_OF_AGGREGATES_NAME, constants::METRIC_VOL_NUMBER_OF_AGGREGATES_HELP),
        &["filer", "volume"]
    ).unwrap();
    pub static ref VOLUME_FLEX_CACHE_ENDPOINT_TYPE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_FLEX_CACHE_ENDPOINT_TYPE_NAME, constants::METRIC_VOL_FLEX_CACHE_ENDPOINT_TYPE_HELP),
        &["filer", "volume", "endpoint_type"]
    ).unwrap();
    pub static ref VOLUME_TYPE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_TYPE_NAME, constants::METRIC_VOL_TYPE_HELP),
        &["filer", "volume", "type"]
    ).unwrap();
    pub static ref VOLUME_CLOUD_RETRIEVAL_POLICY: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_CLOUD_RETRIEVAL_POLICY_NAME, constants::METRIC_VOL_CLOUD_RETRIEVAL_POLICY_HELP),
        &["filer", "volume", "policy"]
    ).unwrap();
    pub static ref VOLUME_QUOTA_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_QUOTA_STATE_NAME, constants::METRIC_VOL_QUOTA_STATE_HELP),
        &["filer", "volume", "state"]
    ).unwrap();
    pub static ref VOLUME_EFFICIENCY_COMPRESSION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_EFFICIENCY_COMPRESSION_NAME, constants::METRIC_VOL_EFFICIENCY_COMPRESSION_HELP),
        &["filer", "volume", "state"]
    ).unwrap();
    pub static ref VOLUME_EFFICIENCY_COMPACTION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_EFFICIENCY_COMPACTION_NAME, constants::METRIC_VOL_EFFICIENCY_COMPACTION_HELP),
        &["filer", "volume", "state"]
    ).unwrap();
    pub static ref VOLUME_EFFICIENCY_DEDUPE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_EFFICIENCY_DEDUPE_NAME, constants::METRIC_VOL_EFFICIENCY_DEDUPE_HELP),
        &["filer", "volume", "state"]
    ).unwrap();
    pub static ref VOLUME_EFFICIENCY_CROSS_VOLUME_DEDUPE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_EFFICIENCY_CROSS_VOLUME_DEDUPE_NAME, constants::METRIC_VOL_EFFICIENCY_CROSS_VOLUME_DEDUPE_HELP),
        &["filer", "volume", "state"]
    ).unwrap();
    pub static ref VOLUME_METRIC_SAMPLE_DURATION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_SAMPLE_DURATION_NAME, constants::METRIC_VOL_METRIC_SAMPLE_DURATION_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_THROUGHPUT_READ: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_THROUGHPUT_READ_NAME, constants::METRIC_VOL_METRIC_THROUGHPUT_READ_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_THROUGHPUT_WRITE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_THROUGHPUT_WRITE_NAME, constants::METRIC_VOL_METRIC_THROUGHPUT_WRITE_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_THROUGHPUT_OTHER: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_THROUGHPUT_OTHER_NAME, constants::METRIC_VOL_METRIC_THROUGHPUT_OTHER_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_THROUGHPUT_TOTAL: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_THROUGHPUT_TOTAL_NAME, constants::METRIC_VOL_METRIC_THROUGHPUT_TOTAL_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_IOPS_READ: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_IOPS_READ_NAME, constants::METRIC_VOL_METRIC_IOPS_READ_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_IOPS_WRITE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_IOPS_WRITE_NAME, constants::METRIC_VOL_METRIC_IOPS_WRITE_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_IOPS_OTHER: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_IOPS_OTHER_NAME, constants::METRIC_VOL_METRIC_IOPS_OTHER_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_IOPS_TOTAL: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_IOPS_TOTAL_NAME, constants::METRIC_VOL_METRIC_IOPS_TOTAL_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_LATENCY_READ: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_LATENCY_READ_NAME, constants::METRIC_VOL_METRIC_LATENCY_READ_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_LATENCY_WRITE: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_LATENCY_WRITE_NAME, constants::METRIC_VOL_METRIC_LATENCY_WRITE_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_LATENCY_OTHER: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_LATENCY_OTHER_NAME, constants::METRIC_VOL_METRIC_LATENCY_OTHER_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_LATENCY_TOTAL: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_LATENCY_TOTAL_NAME, constants::METRIC_VOL_METRIC_LATENCY_TOTAL_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_CLOUD_SAMPLE_DURATION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_CLOUD_SAMPLE_DURATION_NAME, constants::METRIC_VOL_METRIC_CLOUD_SAMPLE_DURATION_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_CLOUD_IOPS_READ: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_CLOUD_IOPS_READ_NAME, constants::METRIC_VOL_METRIC_CLOUD_IOPS_READ_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_CLOUD_IOPS_WRITE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_CLOUD_IOPS_WRITE_NAME, constants::METRIC_VOL_METRIC_CLOUD_IOPS_WRITE_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_CLOUD_IOPS_OTHER: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_CLOUD_IOPS_OTHER_NAME, constants::METRIC_VOL_METRIC_CLOUD_IOPS_OTHER_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_CLOUD_IOPS_TOTAL: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_CLOUD_IOPS_TOTAL_NAME, constants::METRIC_VOL_METRIC_CLOUD_IOPS_TOTAL_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_CLOUD_LATENCY_READ: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_CLOUD_LATENCY_READ_NAME, constants::METRIC_VOL_METRIC_CLOUD_LATENCY_READ_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_CLOUD_LATENCY_WRITE: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_CLOUD_LATENCY_WRITE_NAME, constants::METRIC_VOL_METRIC_CLOUD_LATENCY_WRITE_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_CLOUD_LATENCY_OTHER: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_CLOUD_LATENCY_OTHER_NAME, constants::METRIC_VOL_METRIC_CLOUD_LATENCY_OTHER_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_CLOUD_LATENCY_TOTAL: GaugeVec = GaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_CLOUD_LATENCY_TOTAL_NAME, constants::METRIC_VOL_METRIC_CLOUD_LATENCY_TOTAL_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_FLEXCACHE_SAMPLE_DURATION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_FLEXCACHE_SAMPLE_DURATION_NAME, constants::METRIC_VOL_METRIC_FLEXCACHE_SAMPLE_DURATION_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_FLEXCACHE_CACHE_MISS_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_METRIC_FLEXCACHE_CACHE_MISS_PERCENT_NAME, constants::METRIC_VOL_METRIC_FLEXCACHE_CACHE_MISS_PERCENT_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_ACCESS_TIME_ENABLED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_ACCESS_TIME_ENABLED_NAME, constants::METRIC_VOL_ACCESS_TIME_ENABLED_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_QUEUE_FOR_ENCRYPTION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_QUEUE_FOR_ENCRYPTION_NAME, constants::METRIC_VOL_QUEUE_FOR_ENCRYPTION_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_SNAPLOCK_APPEND_MODE_ENABLED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_SNAPLOCK_APPEND_MODE_ENABLED_NAME, constants::METRIC_VOL_SNAPLOCK_APPEND_MODE_ENABLED_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_SNAPLOCK_LITIGATION_COUNT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_SNAPLOCK_LITIGATION_COUNT_NAME, constants::METRIC_VOL_SNAPLOCK_LITIGATION_COUNT_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_SNAPLOCK_UNSPECIFIED_RETENTION_FILE_COUNT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_SNAPLOCK_UNSPECIFIED_RETENTION_FILE_COUNT_NAME, constants::METRIC_VOL_SNAPLOCK_UNSPECIFIED_RETENTION_FILE_COUNT_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_SNAPLOCK_IS_AUDIT_LOG: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_SNAPLOCK_IS_AUDIT_LOG_NAME, constants::METRIC_VOL_SNAPLOCK_IS_AUDIT_LOG_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_SNAPLOCK_PRIVILEGED_DELETE_TYPE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_SNAPLOCK_PRIVILEGED_DELETE_NAME, constants::METRIC_VOL_SNAPLOCK_PRIVILEGED_DELETE_HELP),
        &["filer", "volume", "type"],
    ).unwrap();
    pub static ref VOLUME_METRIC_SNAPLOCK_TYPE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_SNAPLOCK_TYPE_NAME, constants::METRIC_VOL_SNAPLOCK_TYPE_HELP),
        &["filer", "volume", "type"],
    ).unwrap();

    pub static ref VOLUME_METRIC_MOVEMENT_PERCENT_COMPLETE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_MOVEMENT_PERCENT_COMPLETE_NAME, constants::METRIC_VOL_MOVEMENT_PERCENT_COMPLETE_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_MOVEMENT_CUTOVER_WINDOW: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_MOVEMENT_CUTOVER_WINDOW_NAME, constants::METRIC_VOL_MOVEMENT_CUTOVER_WINDOW_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_MOVEMENT_TIERING_POLICY: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_MOVEMENT_TIERING_POLICY_NAME, constants::METRIC_VOL_MOVEMENT_TIERING_POLICY_HELP),
        &["filer", "volume", "tiering_policy"],
    ).unwrap();
    pub static ref VOLUME_METRIC_MOVEMENT_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_MOVEMENT_STATE_NAME, constants::METRIC_VOL_MOVEMENT_STATE_HELP),
        &["filer", "volume", "state"],
    ).unwrap();

    pub static ref VOLUME_METRIC_STYLE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_STYLE_NAME, constants::METRIC_VOL_STYLE_HELP),
        &["filer", "volume", "style"],
    ).unwrap();

    pub static ref VOLUME_METRIC_ENCRYPTION_TYPE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_ENCRYPTION_TYPE_NAME, constants::METRIC_VOL_ENCRYPTION_TYPE_HELP),
        &["filer", "volume", "type"],
    ).unwrap();
    pub static ref VOLUME_METRIC_ENCRYPTION_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_ENCRYPTION_STATE_NAME, constants::METRIC_VOL_ENCRYPTION_STATE_HELP),
        &["filer", "volume", "state"],
    ).unwrap();
    pub static ref VOLUME_METRIC_ENCRYPTION_ENABLED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_ENCRYPTION_ENABLED_NAME, constants::METRIC_VOL_ENCRYPTION_ENABLED_HELP),
        &["filer", "volume"],
    ).unwrap();

    pub static ref VOLUME_METRIC_TIERING_POLICY: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_TIERING_POLICY_NAME, constants::METRIC_VOL_TIERING_POLICY_HELP),
        &["filer", "volume", "policy"],
    ).unwrap();
    pub static ref VOLUME_METRIC_TIERING_SUPPORTED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_TIERING_SUPPORTED_NAME, constants::METRIC_VOL_TIERING_SUPPORTED_HELP),
        &["filer", "volume"],
    ).unwrap();
    pub static ref VOLUME_METRIC_TIERING_MIN_COOLING_DAYS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_VOL_TIERING_MIN_COOLING_DAYS_NAME, constants::METRIC_VOL_TIERING_MIN_COOLING_DAYS_HELP),
        &["filer", "volume"],
    ).unwrap();
}

// NOTE: macro split is required to suppress the "recursion limit reached while expanding `__lazy_static_internal!`" error
lazy_static! {
    pub static ref VOLUME_METRIC_SPACE_BLOCK_STORAGE_INACTIVE_USER_DATA: IntGaugeVec =
        IntGaugeVec::new(
            Opts::new(
                constants::METRIC_VOL_SPACE_BLOCKSTORAGE_INACTIVE_USER_DATA_NAME,
                constants::METRIC_VOL_SPACE_BLOCKSTORAGE_INACTIVE_USER_DATA_HELP
            ),
            &["filer", "volume"],
        )
        .unwrap();
    pub static ref VOLUME_METRIC_SPACE_OVER_PROVISIONED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_OVER_PROVISIONED_NAME,
            constants::METRIC_VOL_SPACE_OVER_PROVISIONED_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_PERFORMANCE_TIER_FOOTPRINT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_PERFORMANCE_TIER_FOOTPRINT_NAME,
            constants::METRIC_VOL_SPACE_PERFORMANCE_TIER_FOOTPRINT_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_FOOTPRINT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_FOOTPRINT_NAME,
            constants::METRIC_VOL_SPACE_FOOTPRINT_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_CAPACITY_TIER_FOOTPRINT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_CAPACITY_TIER_FOOTPRINT_NAME,
            constants::METRIC_VOL_SPACE_CAPACITY_TIER_FOOTPRINT_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_TOTAL_FOOTPRINT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_TOTAL_FOOTPRINT_NAME,
            constants::METRIC_VOL_SPACE_TOTAL_FOOTPRINT_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_SIZE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_SIZE_NAME,
            constants::METRIC_VOL_SPACE_SIZE_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_LOGICAL_SPACE_REPORTING: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_LOGICAL_SPACE_REPORTING_NAME,
            constants::METRIC_VOL_SPACE_LOGICAL_SPACE_REPORTING_HELP
        ),
        &["filer", "volume",]
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_LOGICAL_SPACE_ENFORCMENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_LOGICAL_SPACE_ENFORCEMENT_NAME,
            constants::METRIC_VOL_SPACE_LOGICAL_SPACE_ENFORCEMENT_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_LOGICAL_SPACE_USED_BY_AFS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_LOGICAL_SPACE_USED_BY_AFS_NAME,
            constants::METRIC_VOL_SPACE_LOGICAL_SPACE_USED_BY_AFS_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_LOGICAL_SPACE_AVAILABLE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_LOGICAL_SPACE_AVAILABLE_NAME,
            constants::METRIC_VOL_SPACE_LOGICAL_SPACE_AVAILABLE_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_USED_NAME,
            constants::METRIC_VOL_SPACE_USED_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_SNAPSHOT_AUTODELETE_ENABLED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_SNAPSHOT_AUTODELETE_ENABLED_NAME,
            constants::METRIC_VOL_SPACE_SNAPSHOT_AUTODELETE_ENABLED_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_SNAPSHOT_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_SNAPSHOT_USED_NAME,
            constants::METRIC_VOL_SPACE_SNAPSHOT_USED_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_SNAPSHOT_RESERVE_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_SNAPSHOT_RESERVE_PERCENT_NAME,
            constants::METRIC_VOL_SPACE_SNAPSHOT_RESERVE_PERCENT_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_METADATA: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_METADATA_NAME,
            constants::METRIC_VOL_SPACE_METADATA_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_AVAILABLE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_AVAILABLE_NAME,
            constants::METRIC_VOL_SPACE_AVAILABLE_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SPACE_LOCAL_TIER_FOOTPRINT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SPACE_LOCAL_TIER_FOOTPRINT_NAME,
            constants::METRIC_VOL_SPACE_LOCAL_TIER_FOOTPRINT_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_ANALYTICS_SCAN_PROGRESS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_ANALYTICS_SCAN_PROGRESS_NAME,
            constants::METRIC_VOL_ANALYTICS_SCAN_PROGRESS_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_ANALYTIC_SUPPORTED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_ANALYTICS_SUPPORTED_NAME,
            constants::METRIC_VOL_ANALYTICS_SUPPORTED_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_ANALYTICS_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_ANALYTICS_STATE_NAME,
            constants::METRIC_VOL_ANALYTICS_STATE_HELP
        ),
        &["filer", "volume", "state"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_GUARANTEE_TYPE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_GUARANTEE_TYPE_NAME,
            constants::METRIC_VOL_GUARANTEE_TYPE_HELP
        ),
        &["filer", "volume", "type"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_GUARANTEE_HONORED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_GUARANTEE_HONORED_NAME,
            constants::METRIC_VOL_GUARANTEE_HONORED_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_IS_SVM_ROOT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_IS_SVM_ROOT_NAME,
            constants::METRIC_VOL_IS_SVM_ROOT_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_USE_MIRRORED_AGGREGATES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_USE_MIRRORED_AGGREGATES_NAME,
            constants::METRIC_VOL_USE_MIRRORED_AGGREGATES_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref VOLUME_METRIC_SNAPMIRROR_PROTECTED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_VOL_SNAPMIRROR_PROTECTED_NAME,
            constants::METRIC_VOL_SNAPMIRROR_PROTECTED_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
}

lazy_static! {
    pub static ref QUOTA_TREE_METRIC_SPACE_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_SPACE_USED_NAME,
            constants::METRIC_TREE_QUOTA_SPACE_USED_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_TREE_METRIC_SPACE_HARD_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_SPACE_HARD_LIMIT_PERCENT_NAME,
            constants::METRIC_TREE_QUOTA_SPACE_HARD_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_TREE_METRIC_SPACE_SOFT_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_SPACE_SOFT_LIMIT_PERCENT_NAME,
            constants::METRIC_TREE_QUOTA_SPACE_SOFT_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_TREE_METRIC_SPACE_HARD_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_SPACE_HARD_LIMIT_NAME,
            constants::METRIC_TREE_QUOTA_SPACE_HARD_LIMIT_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_TREE_METRIC_SPACE_SOFT_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_SPACE_SOFT_LIMIT_NAME,
            constants::METRIC_TREE_QUOTA_SPACE_SOFT_LIMIT_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_TREE_METRIC_FILES_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_FILES_USED_NAME,
            constants::METRIC_TREE_QUOTA_FILES_USED_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_TREE_METRIC_FILES_HARD_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_FILES_HARD_LIMIT_PERCENT_NAME,
            constants::METRIC_TREE_QUOTA_FILES_HARD_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_TREE_METRIC_FILES_SOFT_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_FILES_SOFT_LIMIT_PERCENT_NAME,
            constants::METRIC_TREE_QUOTA_FILES_SOFT_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_TREE_METRIC_FILES_HARD_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_FILES_HARD_LIMIT_NAME,
            constants::METRIC_TREE_QUOTA_FILES_HARD_LIMIT_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_TREE_METRIC_FILES_SOFT_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_TREE_QUOTA_FILES_SOFT_LIMIT_NAME,
            constants::METRIC_TREE_QUOTA_FILES_SOFT_LIMIT_HELP
        ),
        &["filer", "volume", "name"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_SPACE_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_SPACE_USED_NAME,
            constants::METRIC_GROUP_QUOTA_SPACE_USED_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_SPACE_HARD_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_SPACE_HARD_LIMIT_PERCENT_NAME,
            constants::METRIC_GROUP_QUOTA_SPACE_HARD_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_SPACE_SOFT_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_SPACE_SOFT_LIMIT_PERCENT_NAME,
            constants::METRIC_GROUP_QUOTA_SPACE_SOFT_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_SPACE_HARD_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_SPACE_HARD_LIMIT_NAME,
            constants::METRIC_GROUP_QUOTA_SPACE_HARD_LIMIT_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_SPACE_SOFT_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_SPACE_SOFT_LIMIT_NAME,
            constants::METRIC_GROUP_QUOTA_SPACE_SOFT_LIMIT_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_FILES_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_FILES_USED_NAME,
            constants::METRIC_GROUP_QUOTA_FILES_USED_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_FILES_HARD_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_FILES_HARD_LIMIT_PERCENT_NAME,
            constants::METRIC_GROUP_QUOTA_FILES_HARD_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_FILES_SOFT_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_FILES_SOFT_LIMIT_PERCENT_NAME,
            constants::METRIC_GROUP_QUOTA_FILES_SOFT_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_FILES_HARD_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_FILES_HARD_LIMIT_NAME,
            constants::METRIC_GROUP_QUOTA_FILES_HARD_LIMIT_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_GROUP_METRIC_FILES_SOFT_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_GROUP_QUOTA_FILES_SOFT_LIMIT_NAME,
            constants::METRIC_GROUP_QUOTA_FILES_SOFT_LIMIT_HELP
        ),
        &["filer", "volume", "name", "group"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_SPACE_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_SPACE_USED_NAME,
            constants::METRIC_USER_QUOTA_SPACE_USED_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_SPACE_HARD_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_SPACE_HARD_LIMIT_PERCENT_NAME,
            constants::METRIC_USER_QUOTA_SPACE_HARD_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_SPACE_SOFT_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_SPACE_SOFT_LIMIT_PERCENT_NAME,
            constants::METRIC_USER_QUOTA_SPACE_SOFT_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_SPACE_HARD_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_SPACE_HARD_LIMIT_NAME,
            constants::METRIC_USER_QUOTA_SPACE_HARD_LIMIT_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_SPACE_SOFT_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_SPACE_SOFT_LIMIT_NAME,
            constants::METRIC_USER_QUOTA_SPACE_SOFT_LIMIT_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_FILES_USED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_FILES_USED_NAME,
            constants::METRIC_USER_QUOTA_FILES_USED_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_FILES_HARD_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_FILES_HARD_LIMIT_PERCENT_NAME,
            constants::METRIC_USER_QUOTA_FILES_HARD_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_FILES_SOFT_LIMIT_PERCENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_FILES_SOFT_LIMIT_PERCENT_NAME,
            constants::METRIC_USER_QUOTA_FILES_SOFT_LIMIT_PERCENT_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_FILES_HARD_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_FILES_HARD_LIMIT_NAME,
            constants::METRIC_USER_QUOTA_FILES_HARD_LIMIT_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
    pub static ref QUOTA_USER_METRIC_FILES_SOFT_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_USER_QUOTA_FILES_SOFT_LIMIT_NAME,
            constants::METRIC_USER_QUOTA_FILES_SOFT_LIMIT_HELP
        ),
        &["filer", "volume", "name", "user"],
    )
    .unwrap();
}

lazy_static! {
    pub static ref CHASSIS_SHELVES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CHASSIS_SHELVES_NAME,
            constants::METRIC_CHASSIS_SHELVES_HELP
        ),
        &["filer", "chassis"],
    )
    .unwrap();
    pub static ref CHASSIS_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CHASSIS_STATE_NAME,
            constants::METRIC_CHASSIS_STATE_HELP
        ),
        &["filer", "chassis", "state"],
    )
    .unwrap();
    pub static ref CHASSIS_NODES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CHASSIS_NODES_NAME,
            constants::METRIC_CHASSIS_NODES_HELP
        ),
        &["filer", "chassis"],
    )
    .unwrap();
    pub static ref CHASSIS_FRU_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CHASSIS_FRU_STATE_NAME,
            constants::METRIC_CHASSIS_FRU_STATE_HELP
        ),
        &["filer", "chassis", "fru", "type", "state"],
    )
    .unwrap();
    pub static ref CHASSIS_USB_SUPPORTED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CHASSIS_USB_SUPPORTED_NAME,
            constants::METRIC_CHASSIS_USB_SUPPORTED_HELP
        ),
        &["filer", "chassis"]
    )
    .unwrap();
    pub static ref CHASSIS_USB_ENABLED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CHASSIS_USB_ENABLED_NAME,
            constants::METRIC_CHASSIS_USB_ENABLED_HELP
        ),
        &["filer", "chassis"]
    )
    .unwrap();
    pub static ref CHASSIS_USB_PORT_CONNECTED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CHASSIS_USB_CONNECTED_STATE_NAME,
            constants::METRIC_CHASSIS_USB_CONNECTED_STATE_HELP
        ),
        &["filer", "chassis", "state"],
    )
    .unwrap();
}

lazy_static! {
    pub static ref CLUSTER_JOB_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_JOBS_STATE_NAME,
            constants::METRIC_JOBS_STATE_HELP
        ),
        &["filer", "state"],
    )
    .unwrap();
}

lazy_static! {
    pub static ref ETHERNET_SPEED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_ETH_SPEED_NAME,
            constants::METRIC_ETH_SPEED_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_ENABLED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_ETH_ENABLED_NAME,
            constants::METRIC_ETH_ENABLED_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_MTU: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_ETH_MTU_NAME,
            constants::METRIC_ETH_MTU_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_UP: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::METRIC_ETH_UP_NAME, constants::METRIC_ETH_UP_HELP),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_TYPE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_ETH_TYPE_NAME,
            constants::METRIC_ETH_TYPE_HELP
        ),
        &["filer", "node", "device", "type"],
    )
    .unwrap();
    pub static ref ETHERNET_RX: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_ETH_RECV_NAME,
            constants::METRIC_ETH_RECV_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_TX: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_ETH_TRANSMIT_NAME,
            constants::METRIC_ETH_TRANSMIT_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_RX_ERROR: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_ETH_RX_ERROR_NAME,
            constants::METRIC_ETH_RX_ERROR_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_RX_DISCARD: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_ETH_RX_DISCARD_NAME,
            constants::METRIC_ETH_RX_DISCARD_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_RX_PACKET: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_ETH_RX_PACKET_NAME,
            constants::METRIC_ETH_RX_PACKET_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_TX_ERROR: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_ETH_TX_ERROR_NAME,
            constants::METRIC_ETH_TX_ERROR_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_TX_DISCARD: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_ETH_TX_DISCARD_NAME,
            constants::METRIC_ETH_TX_DISCARD_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_TX_PACKET: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_ETH_TX_PACKET_NAME,
            constants::METRIC_ETH_TX_PACKET_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref ETHERNET_LINK_DOWN: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_ETH_LINK_DOWN_NAME,
            constants::METRIC_ETH_LINK_DOWN_HELP
        ),
        &["filer", "node", "device"],
    )
    .unwrap();
}

lazy_static! {
    pub static ref FC_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_FC_STATE_NAME,
            constants::METRIC_FC_STATE_HELP
        ),
        &["filer", "name", "device", "state"],
    )
    .unwrap();
    pub static ref FC_ENABLED: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_FC_ENABLED_NAME,
            constants::METRIC_FC_ENABLED_HELP
        ),
        &["filer", "name", "device"],
    )
    .unwrap();
    pub static ref FC_RX: IntCounterVec = IntCounterVec::new(
        Opts::new(constants::METRIC_FC_RX_NAME, constants::METRIC_FC_RX_HELP),
        &["filer", "name", "device"]
    )
    .unwrap();
    pub static ref FC_TX: IntCounterVec = IntCounterVec::new(
        Opts::new(constants::METRIC_FC_TX_NAME, constants::METRIC_FC_TX_HELP),
        &["filer", "node", "device"],
    )
    .unwrap();
    pub static ref FC_PHYS_PROTO: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_FC_PHYS_PROTO_NAME,
            constants::METRIC_FC_PHYS_PROTO_HELP
        ),
        &["filer", "node", "device", "protocol"],
    )
    .unwrap();
}

lazy_static! {
    pub static ref CIFS_PROTOCOLS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_PROTOCOLS_NAME,
            constants::METRIC_CIFS_PROTOCOLS_HELP
        ),
        &["filer", "smb_protocol"],
    )
    .unwrap();
    pub static ref CIFS_SMB_ENCRYPTION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_SMB_ENCRYPTION_NAME,
            constants::METRIC_CIFS_SMB_ENCRYPTION_HELP
        ),
        &["filer", "smb_encryption"],
    )
    .unwrap();
    pub static ref CIFS_CONTINUOUS_AVAILABILITY: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_CONTINUOUS_AVAIABILITY_NAME,
            constants::METRIC_CIFS_CONTINUOUS_AVAIABILITY_HELP
        ),
        &["filer", "continuous_availability"],
    )
    .unwrap();
    pub static ref CIFS_OPEN_FILES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_OPEN_FILES_NAME,
            constants::METRIC_CIFS_OPEN_FILES_HELP
        ),
        &["filer"],
    )
    .unwrap();
    pub static ref CIFS_OPEN_SHARES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_OPEN_SHARES_NAME,
            constants::METRIC_CIFS_OPEN_SHARES_HELP
        ),
        &["filer"],
    )
    .unwrap();
    pub static ref CIFS_OPEN_OTHER: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_OPEN_OTHER_NAME,
            constants::METRIC_CIFS_OPEN_OTHER_HELP
        ),
        &["filer"],
    )
    .unwrap();
    pub static ref CIFS_AUTHENTICATION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_AUTHENTICATION_NAME,
            constants::METRIC_CIFS_AUTHENTICATION_HELP
        ),
        &["filer", "authentication"],
    )
    .unwrap();
    pub static ref CIFS_SMB_SIGNING: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_SMB_SIGNING_NAME,
            constants::METRIC_CIFS_SMB_SIGNING_HELP
        ),
        &["filer", "smb_signing"],
    )
    .unwrap();
    pub static ref CIFS_USER: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_USER_NANE,
            constants::METRIC_CIFS_USER_HELP
        ),
        &["filer", "user"],
    )
    .unwrap();
    pub static ref CIFS_MAPPED_UNIX_USER: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_MAPPED_UNIX_USER_NANE,
            constants::METRIC_CIFS_MAPPED_UNIX_USER_HELP
        ),
        &["filer", "mapped_unix_user"],
    )
    .unwrap();
    pub static ref CIFS_CLIENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_CLIENT_NAME,
            constants::METRIC_CIFS_CLIENT_HELP
        ),
        &["filer", "client_ip"],
    )
    .unwrap();
    pub static ref CIFS_VOLUME: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_VOLUME_NAME,
            constants::METRIC_CIFS_VOLUME_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref CIFS_LARGE_MTU: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_LARGE_MTU_NAME,
            constants::METRIC_CIFS_LARGE_MTU_HELP
        ),
        &["filer", "large_mtu"],
    )
    .unwrap();
    pub static ref CIFS_CONNECTION: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_CIFS_CONNECTIONS_NAME,
            constants::METRIC_CIFS_CONNECTIONS_HELP
        ),
        &["filer"],
    )
    .unwrap();
}

lazy_static! {
    pub static ref NFS_PROTOCOL: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_NFS_PROTOCOL_NAME,
            constants::METRIC_NFS_PROTOCOL_HELP
        ),
        &["filer", "protocol"],
    )
    .unwrap();
    pub static ref NFS_VOLUME: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_NFS_VOLUME_NAME,
            constants::METRIC_NFS_VOLUME_HELP
        ),
        &["filer", "volume"],
    )
    .unwrap();
    pub static ref NFS_LOCAL_COUNT: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_NFS_LOCAL_REQUEST_COUNT_NAME,
            constants::METRIC_NFS_LOCAL_REQUEST_COUNT_HELP
        ),
        &["filer"],
    )
    .unwrap();
    pub static ref NFS_REMOTE_COUNT: IntCounterVec = IntCounterVec::new(
        Opts::new(
            constants::METRIC_NFS_REMOTE_REQUEST_COUNT_NAME,
            constants::METRIC_NFS_REMOTE_REQUEST_COUNT_HELP
        ),
        &["filer"],
    )
    .unwrap();
    pub static ref NFS_CLIENT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_NFS_CLIENT_NAME,
            constants::METRIC_NFS_CLIENT_HELP
        ),
        &["filer", "client_ip"],
    )
    .unwrap();
}

pub fn register_nfs_metrics() {
    REGISTRY.register(Box::new(NFS_PROTOCOL.clone())).unwrap();
    REGISTRY.register(Box::new(NFS_VOLUME.clone())).unwrap();
    REGISTRY
        .register(Box::new(NFS_LOCAL_COUNT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(NFS_REMOTE_COUNT.clone()))
        .unwrap();
    REGISTRY.register(Box::new(NFS_CLIENT.clone())).unwrap();
}

pub fn register_cifs_metrics() {
    REGISTRY.register(Box::new(CIFS_PROTOCOLS.clone())).unwrap();
    REGISTRY
        .register(Box::new(CIFS_SMB_ENCRYPTION.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CIFS_CONTINUOUS_AVAILABILITY.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CIFS_OPEN_FILES.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CIFS_OPEN_SHARES.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CIFS_OPEN_OTHER.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CIFS_AUTHENTICATION.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CIFS_SMB_SIGNING.clone()))
        .unwrap();
    REGISTRY.register(Box::new(CIFS_USER.clone())).unwrap();
    REGISTRY
        .register(Box::new(CIFS_MAPPED_UNIX_USER.clone()))
        .unwrap();
    REGISTRY.register(Box::new(CIFS_CLIENT.clone())).unwrap();
    REGISTRY.register(Box::new(CIFS_VOLUME.clone())).unwrap();
    REGISTRY.register(Box::new(CIFS_LARGE_MTU.clone())).unwrap();
    REGISTRY
        .register(Box::new(CIFS_CONNECTION.clone()))
        .unwrap();
}

pub fn register_fibrechannel_metrics() {
    REGISTRY.register(Box::new(FC_STATE.clone())).unwrap();
    REGISTRY.register(Box::new(FC_ENABLED.clone())).unwrap();
    REGISTRY.register(Box::new(FC_RX.clone())).unwrap();
    REGISTRY.register(Box::new(FC_TX.clone())).unwrap();
    REGISTRY.register(Box::new(FC_PHYS_PROTO.clone())).unwrap();
}

pub fn register_ethernet_metrics() {
    REGISTRY.register(Box::new(ETHERNET_SPEED.clone())).unwrap();
    REGISTRY
        .register(Box::new(ETHERNET_ENABLED.clone()))
        .unwrap();
    REGISTRY.register(Box::new(ETHERNET_MTU.clone())).unwrap();
    REGISTRY.register(Box::new(ETHERNET_UP.clone())).unwrap();
    REGISTRY.register(Box::new(ETHERNET_TYPE.clone())).unwrap();
    REGISTRY.register(Box::new(ETHERNET_RX.clone())).unwrap();
    REGISTRY.register(Box::new(ETHERNET_TX.clone())).unwrap();
    REGISTRY
        .register(Box::new(ETHERNET_RX_ERROR.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(ETHERNET_RX_DISCARD.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(ETHERNET_RX_PACKET.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(ETHERNET_TX_ERROR.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(ETHERNET_TX_DISCARD.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(ETHERNET_TX_PACKET.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(ETHERNET_LINK_DOWN.clone()))
        .unwrap();
}

pub fn register_job_metrics() {
    REGISTRY
        .register(Box::new(CLUSTER_JOB_STATE.clone()))
        .unwrap();
}

pub fn register_chassis_metrics() {
    REGISTRY
        .register(Box::new(CHASSIS_SHELVES.clone()))
        .unwrap();
    REGISTRY.register(Box::new(CHASSIS_STATE.clone())).unwrap();
    REGISTRY.register(Box::new(CHASSIS_NODES.clone())).unwrap();
    REGISTRY
        .register(Box::new(CHASSIS_FRU_STATE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CHASSIS_USB_SUPPORTED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CHASSIS_USB_ENABLED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CHASSIS_USB_PORT_CONNECTED.clone()))
        .unwrap();
}

pub fn register_quota_metrics() {
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_SPACE_USED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_SPACE_HARD_LIMIT_PERCENT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_SPACE_SOFT_LIMIT_PERCENT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_SPACE_HARD_LIMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_SPACE_SOFT_LIMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_FILES_USED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_FILES_HARD_LIMIT_PERCENT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_FILES_SOFT_LIMIT_PERCENT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_FILES_HARD_LIMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_TREE_METRIC_FILES_SOFT_LIMIT.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(QUOTA_GROUP_METRIC_SPACE_USED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(
            QUOTA_GROUP_METRIC_SPACE_HARD_LIMIT_PERCENT.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(
            QUOTA_GROUP_METRIC_SPACE_SOFT_LIMIT_PERCENT.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_GROUP_METRIC_SPACE_HARD_LIMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_GROUP_METRIC_SPACE_SOFT_LIMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_GROUP_METRIC_FILES_USED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(
            QUOTA_GROUP_METRIC_FILES_HARD_LIMIT_PERCENT.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(
            QUOTA_GROUP_METRIC_FILES_SOFT_LIMIT_PERCENT.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_GROUP_METRIC_FILES_HARD_LIMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_GROUP_METRIC_FILES_SOFT_LIMIT.clone()))
        .unwrap();
}

pub fn register_aggregate_metrics() {
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

    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_SPACE_USED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_SPACE_HARD_LIMIT_PERCENT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_SPACE_SOFT_LIMIT_PERCENT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_SPACE_HARD_LIMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_SPACE_SOFT_LIMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_FILES_USED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_FILES_HARD_LIMIT_PERCENT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_FILES_SOFT_LIMIT_PERCENT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_FILES_HARD_LIMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(QUOTA_USER_METRIC_FILES_SOFT_LIMIT.clone()))
        .unwrap();
}

pub fn register_volume_metrics() {
    REGISTRY
        .register(Box::new(VOLUME_FILES_MAX.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_FILES_USED.clone()))
        .unwrap();

    REGISTRY.register(Box::new(VOLUME_STATE.clone())).unwrap();

    REGISTRY
        .register(Box::new(VOLUME_ERROR_STATE.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_AUTOSIZE_MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_AUTOSIZE_MAX.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_AUTOSIZE_SHRINK_THRESHOLD.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_AUTOSIZE_GROW_THRESHOLD.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_AUTOSIZE_MODE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_IS_OBJECT_STORE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_NUMBER_OF_AGGREGATES.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_FLEX_CACHE_ENDPOINT_TYPE.clone()))
        .unwrap();
    REGISTRY.register(Box::new(VOLUME_TYPE.clone())).unwrap();
    REGISTRY
        .register(Box::new(VOLUME_CLOUD_RETRIEVAL_POLICY.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_QUOTA_STATE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_EFFICIENCY_COMPRESSION.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_EFFICIENCY_COMPACTION.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_EFFICIENCY_DEDUPE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_EFFICIENCY_CROSS_VOLUME_DEDUPE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SAMPLE_DURATION.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_THROUGHPUT_READ.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_THROUGHPUT_WRITE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_THROUGHPUT_OTHER.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_THROUGHPUT_TOTAL.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_IOPS_READ.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_IOPS_WRITE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_IOPS_OTHER.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_IOPS_TOTAL.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_LATENCY_READ.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_LATENCY_WRITE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_LATENCY_OTHER.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_LATENCY_TOTAL.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_CLOUD_SAMPLE_DURATION.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_CLOUD_IOPS_READ.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_CLOUD_IOPS_WRITE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_CLOUD_IOPS_OTHER.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_CLOUD_IOPS_TOTAL.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_CLOUD_LATENCY_READ.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_CLOUD_LATENCY_WRITE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_CLOUD_LATENCY_OTHER.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_CLOUD_LATENCY_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_FLEXCACHE_SAMPLE_DURATION.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_FLEXCACHE_CACHE_MISS_PERCENT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_ACCESS_TIME_ENABLED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_QUEUE_FOR_ENCRYPTION.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_SNAPLOCK_APPEND_MODE_ENABLED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SNAPLOCK_LITIGATION_COUNT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SNAPLOCK_UNSPECIFIED_RETENTION_FILE_COUNT.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SNAPLOCK_IS_AUDIT_LOG.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SNAPLOCK_PRIVILEGED_DELETE_TYPE.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SNAPLOCK_TYPE.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_MOVEMENT_PERCENT_COMPLETE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_MOVEMENT_CUTOVER_WINDOW.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_MOVEMENT_TIERING_POLICY.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_MOVEMENT_STATE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_STYLE.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_ENCRYPTION_TYPE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_ENCRYPTION_STATE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_ENCRYPTION_ENABLED.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_TIERING_POLICY.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_TIERING_SUPPORTED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_TIERING_MIN_COOLING_DAYS.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SPACE_BLOCK_STORAGE_INACTIVE_USER_DATA.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SPACE_OVER_PROVISIONED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SPACE_PERFORMANCE_TIER_FOOTPRINT.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SPACE_FOOTPRINT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SPACE_CAPACITY_TIER_FOOTPRINT.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SPACE_TOTAL_FOOTPRINT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SPACE_SIZE.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SPACE_LOGICAL_SPACE_REPORTING.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SPACE_LOGICAL_SPACE_ENFORCMENT.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SPACE_LOGICAL_SPACE_USED_BY_AFS.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SPACE_LOGICAL_SPACE_AVAILABLE.clone(),
        ))
        .unwrap();

    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SPACE_SNAPSHOT_AUTODELETE_ENABLED.clone(),
        ))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SPACE_SNAPSHOT_USED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(
            VOLUME_METRIC_SPACE_SNAPSHOT_RESERVE_PERCENT.clone(),
        ))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_SPACE_METADATA.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SPACE_AVAILABLE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SPACE_LOCAL_TIER_FOOTPRINT.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_ANALYTICS_SCAN_PROGRESS.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_ANALYTIC_SUPPORTED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_ANALYTICS_STATE.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_GUARANTEE_TYPE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_GUARANTEE_HONORED.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(VOLUME_METRIC_IS_SVM_ROOT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_USE_MIRRORED_AGGREGATES.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(VOLUME_METRIC_SNAPMIRROR_PROTECTED.clone()))
        .unwrap();
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
    } else {
        info!("Aggregate information has been disabled for {}", filer.name);
    }

    if filer.targets_mask & constants::TARGET_QUOTAS == constants::TARGET_QUOTAS {
        info!("Requesting quota information from {}", filer.name);
        if let Err(e) = quotas::update_quotas(filer, client) {
            error!(
                "Unable to update quota statistics for {} - {}",
                filer.name, e
            );
        }
    } else {
        info!("Quota information has been disabled for {}", filer.name);
    }

    if filer.targets_mask & constants::TARGET_VOLUMES == constants::TARGET_VOLUMES {
        info!("Requesting volume information from {}", filer.name);
        if let Err(e) = volumes::update_volumes(filer, client) {
            error!(
                "Unable to update volume statistics for {} - {}",
                filer.name, e
            );
        }
    } else {
        info!("Volume information has been disabled for {}", filer.name);
    }

    if filer.targets_mask & constants::TARGET_CHASSIS == constants::TARGET_CHASSIS {
        info!("Requesting cluster chassis information from {}", filer.name);
        if let Err(e) = chassis::update_chassis(filer, client) {
            error!(
                "Unable to update cluster chassis statistics for {} - {}",
                filer.name, e
            );
        }
    } else {
        info!(
            "Cluster chassis information has been disabled for {}",
            filer.name
        );
    }

    if filer.targets_mask & constants::TARGET_JOBS == constants::TARGET_JOBS {
        info!("Requesting cluster job information from {}", filer.name);
        if let Err(e) = jobs::update_jobs(filer, client) {
            error!(
                "Unable to update cluster job statistics for {} - {}",
                filer.name, e
            );
        }
    } else {
        info!(
            "Cluster job information has been disabled for {}",
            filer.name
        );
    }

    if filer.targets_mask & constants::TARGET_ETHERNET == constants::TARGET_ETHERNET {
        info!("Requesting ethernet port information from {}", filer.name);
        if let Err(e) = ethernet::update_ethernet(filer, client) {
            error!(
                "Unable to update ethernet port statistics for {} - {}",
                filer.name, e
            );
        }
    } else {
        info!(
            "Ethernet port information has been disabled for {}",
            filer.name
        );
    }

    if filer.targets_mask & constants::TARGET_FIBRECHANNEL == constants::TARGET_FIBRECHANNEL {
        info!(
            "Requesting fibrechannel port information from {}",
            filer.name
        );
        if let Err(e) = fibrechannel::update_fibrechannel(filer, client) {
            error!(
                "Unable to update fibrechannel port statistics for {} - {}",
                filer.name, e
            );
        }
    } else {
        info!(
            "Fibrechannel port information has been disabled for {}",
            filer.name
        );
    }

    if filer.targets_mask & constants::TARGET_CIFS == constants::TARGET_CIFS {
        info!("Requesting CIFS protocol information from {}", filer.name);

        let mut cifs_mapped_user = false;
        if filer.targets_mask & constants::TARGET_CIFS_MAPPED_USER
            == constants::TARGET_CIFS_MAPPED_USER
        {
            cifs_mapped_user = true
        }

        let mut cifs_user = false;
        if filer.targets_mask & constants::TARGET_CIFS_USER == constants::TARGET_CIFS_USER {
            cifs_user = true
        }

        let mut cifs_client_ip = false;
        if filer.targets_mask & constants::TARGET_CIFS_CLIENT_IP == constants::TARGET_CIFS_CLIENT_IP
        {
            cifs_client_ip = true
        }

        if let Err(e) =
            cifs::update_cifs(filer, client, cifs_client_ip, cifs_mapped_user, cifs_user)
        {
            error!(
                "Unable to update CIFS protocol statistics for {} - {}",
                filer.name, e
            );
        }
    } else {
        info!(
            "CIFS protocol information has been disabled for {}",
            filer.name
        );
    }

    if filer.targets_mask & constants::TARGET_NFS == constants::TARGET_NFS {
        info!("Requesting NFS protocol information from {}", filer.name);

        let mut nfs_client_ip = false;
        if filer.targets_mask & constants::TARGET_NFS_CLIENT_IP == constants::TARGET_NFS_CLIENT_IP {
            nfs_client_ip = true
        }

        if let Err(e) = nfs::update_nfs(filer, client, nfs_client_ip) {
            error!(
                "Unable to update NFS protocol statistics for {} - {}",
                filer.name, e
            );
        }
    } else {
        info!(
            "NFS protocol information has been disabled for {}",
            filer.name
        );
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
