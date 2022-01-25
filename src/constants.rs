pub const NAME: &str = "prometheus-netapp-exporter";
pub const VERSION: &str = "0.1.1-20220124";
pub const DEFAULT_INSECURE_SSL: bool = false;
pub const DEFAULT_TIMEOUT: u64 = 60;
pub const DEFAULT_PROMETHEUS_ADDRESS: &str = "localhost:9988";
const REPO_URL: &str = "https://ypbind.de/cgit/prometheus-netapp-exporter/";

pub fn generate_default_user_agent() -> String {
    format!("{}/{} ({})", NAME, VERSION, REPO_URL)
}

pub const ROOT_HTML: &str = "<html>\n<head><title>NetApp exporter</title></head>\n<body>\n<h1>NetApp exporter</h1>\n<p><a href=\"/metrics\">Metrics</a></p>\n</body>\n</html>\n";
pub const METRICS_PATH: &str = "/metrics";
pub const HTTP_CLIENT_TIMEOUT: u64 = 15;

pub const API_AGGREGATES: &str = "/api/storage/aggregates";
pub const API_VOLUMES: &str = "/api/storage/volumes";

pub const TARGET_AGGREGATES: u64 = 0x0000000000000001;
pub const TARGET_VOLUMES: u64 = 0x0000000000000002;

pub const METRIC_AGGR_FOOTPRINT_NAME: &str = "netapp_aggregate_footprint_bytes";
pub const METRIC_AGGR_FOOTPRINT_HELP: &str =
    "A summation of volume footprints (including volume guarantees), in bytes";

pub const METRIC_AGGR_BLOCK_STORAGE_SIZE_NAME: &str = "netapp_aggregate_block_storage_size_bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_SIZE_HELP: &str =
    "Total usable space in bytes, not including WAFL reserve and aggregate Snapshot copy reserve";
pub const METRIC_AGGR_BLOCK_STORAGE_USED_NAME: &str = "netapp_aggregate_block_storage_used_bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_USED_HELP: &str =
    "Space used or reserved in bytes includes volume guarantees and aggregate metadata.";
pub const METRIC_AGGR_BLOCK_STORAGE_AVAILABLE_NAME: &str =
    "netapp_aggregate_block_storage_available_bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_AVAILABLE_HELP: &str = "Space available in bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_FULL_THRESHOLD_NAME: &str =
    "netapp_aggregate_block_storage_full_threshold_percent";
pub const METRIC_AGGR_BLOCK_STORAGE_FULL_THRESHOLD_HELP: &str =
    "The aggregate used percentage at which monitor.volume.full EMS is generated";

pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_LOGICAL_USED_NAME: &str =
    "netapp_aggregate_block_storage_efficiency_logical_used_bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_LOGICAL_USED_HELP: &str =
    "Logical used including snapshots";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_SAVINGS_NAME: &str =
    "netapp_aggregate_block_storage_efficiency_savings_bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_SAVINGS_HELP: &str =
    "Space saved by storage efficiencies including snapshots";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_RATIO_NAME: &str =
    "netapp_aggregate_block_storage_efficiency_ratio";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_RATIO_HELP: &str =
    "Data reduction ratio including snapshots";

pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_LOGICAL_USED_NAME: &str =
    "netapp_aggregate_block_storage_efficiency_without_snapshots_logical_used_bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_LOGICAL_USED_HELP: &str =
    "Logical used without snapshots";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_SAVINGS_NAME: &str =
    "netapp_aggregate_block_storage_efficiency_without_snapshots_savings_bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_SAVINGS_HELP: &str =
    "Space saved by storage efficiencies without snapshots";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_RATIO_NAME: &str =
    "netapp_aggregate_block_storage_efficiency_without_snapshots_ratio";
pub const METRIC_AGGR_BLOCK_STORAGE_EFFICIENCY_WO_SNAPSHOTS_RATIO_HELP: &str =
    "Data reduction ratio without snapshots";

pub const METRIC_AGGR_CLOUD_STORAGE_USED_NAME: &str = "netapp_aggregate_cloud_storage_used_bytes";
pub const METRIC_AGGR_CLOUD_STORAGE_USED_HELP: &str = "Used space in bytes in the cloud store";

pub const METRIC_AGGR_BLOCK_STORAGE_PLEXES_NAME: &str = "netapp_aggregate_block_storage_plexes";
pub const METRIC_AGGR_BLOCK_STORAGE_PLEXES_HELP: &str = "The number of plexes in the aggregate";

pub const METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_ENABLED_NAME: &str =
    "netapp_aggregate_block_storage_hybrid_cache_enabled_info";
pub const METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_ENABLED_HELP: &str =
    "Specifies whether the aggregate uses HDDs with SSDs as a cache";
pub const METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_DISK_USED_NAME: &str =
    "netapp_aggregate_block_storage_hybrid_cache_disk_used";
pub const METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_DISK_USED_HELP: &str =
    "Number of disks used in the cache tier of the aggregate";
pub const METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_SIZE_NAME: &str =
    "netapp_aggregate_block_storage_hybrid_cache_size_bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_SIZE_HELP: &str =
    "Total usable space in bytes of SSD cache";
pub const METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_USED_NAME: &str =
    "netapp_aggregate_block_storage_hybrid_cache_used_bytes";
pub const METRIC_AGGR_BLOCK_STORAGE_HYBRID_CACHE_USED_HELP: &str =
    "Space used in bytes of SSD cache";

pub const METRIC_AGGR_BLOCK_STORAGE_PRIMARY_DISK_COUNT_NAME: &str =
    "netapp_aggregate_block_storage_primary_disks";
pub const METRIC_AGGR_BLOCK_STORAGE_PRIMARY_DISK_COUNT_HELP: &str = "Number of disks used in the aggregate including parity disks, but excluding disks in the hybrid cache";
pub const METRIC_AGGR_BLOCK_STORAGE_PRIMARY_RAID_SIZE_NAME: &str =
    "netapp_aggregate_block_storage_primary_raid_size";
pub const METRIC_AGGR_BLOCK_STORAGE_PRIMARY_RAID_SIZE_HELP: &str =
    "The maximum number of disks that can be included in a RAID group";

pub const METRIC_AGGR_BLOCK_STORAGE_MIRROR_ENABLED_NAME: &str =
    "netapp_aggregate_block_storage_mirror_enabled_info";
pub const METRIC_AGGR_BLOCK_STORAGE_MIRROR_ENABLED_HELP: &str = "Aggregate is SyncMirror protected";
pub const METRIC_AGGR_BLOCK_STORAGE_MIRROR_STATE_NAME: &str =
    "netapp_aggregate_block_storage_mirror_state_info";
pub const METRIC_AGGR_BLOCK_STORAGE_MIRROR_STATE_HELP: &str = "Current state of SyncMirror";

pub const METRIC_AGGR_STATE_NAME: &str = "netapp_aggregate_state_info";
pub const METRIC_AGGR_STATE_HELP: &str = "Operational state of the aggregate";

pub const METRIC_AGGR_METRIC_THROUGHPUT_READ_NAME: &str =
    "netapp_aggregate_metric_throughput_read_bytes_per_second";
pub const METRIC_AGGR_METRIC_THROUGHPUT_READ_HELP: &str =
    "Performance metric for read I/O operations in bytes per second";
pub const METRIC_AGGR_METRIC_THROUGHPUT_WRITE_NAME: &str =
    "netapp_aggregate_metric_throughput_write_bytes_per_second";
pub const METRIC_AGGR_METRIC_THROUGHPUT_WRITE_HELP: &str =
    "Performance metric for write I/O operations in bytes per second";
pub const METRIC_AGGR_METRIC_THROUGHPUT_OTHER_NAME: &str =
    "netapp_aggregate_metric_throughput_other_bytes_per_second";
pub const METRIC_AGGR_METRIC_THROUGHPUT_OTHER_HELP: &str =
    "Performance metric for other I/O operations in bytes per second";
pub const METRIC_AGGR_METRIC_THROUGHPUT_TOTAL_NAME: &str =
    "netapp_aggregate_metric_throughput_total_bytes_per_second";
pub const METRIC_AGGR_METRIC_THROUGHPUT_TOTAL_HELP: &str =
    "Performance metric for all I/O operations in bytes per second";

pub const METRIC_AGGR_METRIC_LATENCY_READ_NAME: &str =
    "netapp_aggregate_metric_latency_read_seconds";
pub const METRIC_AGGR_METRIC_LATENCY_READ_HELP: &str =
    "The round trip latency in microseconds observed at the storage object for read operations";
pub const METRIC_AGGR_METRIC_LATENCY_WRITE_NAME: &str =
    "netapp_aggregate_metric_latency_write_seconds";
pub const METRIC_AGGR_METRIC_LATENCY_WRITE_HELP: &str =
    "The round trip latency in microseconds observed at the storage object for write operations";
pub const METRIC_AGGR_METRIC_LATENCY_OTHER_NAME: &str =
    "netapp_aggregate_metric_latency_other_seconds";
pub const METRIC_AGGR_METRIC_LATENCY_OTHER_HELP: &str =
    "The round trip latency in microseconds observed at the storage object for other operations";
pub const METRIC_AGGR_METRIC_LATENCY_TOTAL_NAME: &str =
    "netapp_aggregate_metric_latency_total_seconds";
pub const METRIC_AGGR_METRIC_LATENCY_TOTAL_HELP: &str =
    "The round trip latency in microseconds observed at the storage object for all operations";

pub const METRIC_AGGR_METRIC_IOPS_READ_NAME: &str =
    "netapp_aggregate_metric_iops_read_iops_per_second";
pub const METRIC_AGGR_METRIC_IOPS_READ_HELP: &str =
    "The rate of I/O operations observed at the storage object for read operations";
pub const METRIC_AGGR_METRIC_IOPS_WRITE_NAME: &str =
    "netapp_aggregate_metric_iops_write_iops_per_second";
pub const METRIC_AGGR_METRIC_IOPS_WRITE_HELP: &str =
    "The rate of I/O operations observed at the storage object for write operations";
pub const METRIC_AGGR_METRIC_IOPS_OTHER_NAME: &str =
    "netapp_aggregate_metric_iops_other_iops_per_second";
pub const METRIC_AGGR_METRIC_IOPS_OTHER_HELP: &str =
    "The rate of I/O operations observed at the storage object for other operations";
pub const METRIC_AGGR_METRIC_IOPS_TOTAL_NAME: &str =
    "netapp_aggregate_metric_iops_total_iops_per_second";
pub const METRIC_AGGR_METRIC_IOPS_TOTAL_HELP: &str =
    "The rate of I/O operations observed at the storage object for all operations";

pub const METRIC_AGGR_METRIC_SAMPLE_DURATION_NAME: &str =
    "netapp_aggregate_metric_sample_duration_seconds";
pub const METRIC_AGGR_METRIC_SAMPLE_DURATION_HELP: &str =
    "The duration over which the sample are calculated";

// Volume metrics
pub const METRIC_VOL_FILES_MAXIMUM_NAME: &str = "netapp_volumes_files_maximum";
pub const METRIC_VOL_FILES_MAXIMUM_HELP: &str =
    "The maximum number of files for user-visible data allowed on the volume";
pub const METRIC_VOL_FILES_USED_NAME: &str = "netapp_volumes_files_used";
pub const METRIC_VOL_FILES_USED_HELP: &str =
    "Number of files used for user-visible data on the volume";

pub const METRIC_VOL_STATE_NAME: &str = "netapp_volume_state_info";
pub const METRIC_VOL_STATE_HELP: &str = "Volume state";

pub const METRIC_VOL_ERROR_STATE_NAME: &str = "netapp_volume_error_state_info";
pub const METRIC_VOL_ERROR_STATE_HELP: &str = "Reason why the volume is in an error state";

pub const METRIC_VOL_AUTOSIZE_MINIMUM_NAME: &str = "netapp_volume_autosize_minimum_bytes";
pub const METRIC_VOL_AUTOSIZE_MINIMUM_HELP: &str =
    "Minimum size in bytes up to which the volume shrinks automatically";
pub const METRIC_VOL_AUTOSIZE_MAXIMUM_NAME: &str = "netapp_volume_autosize_maximum_bytes";
pub const METRIC_VOL_AUTOSIZE_MAXIMUM_HELP: &str =
    "Maximum size in bytes up to which a volume grows automatically";
pub const METRIC_VOL_AUTOSIZE_SHRINK_THRESHOLD_NAME: &str =
    "netapp_volume_autosize_shrink_threshold_percent";
pub const METRIC_VOL_AUTOSIZE_SHRINK_THRESHOLD_HELP: &str =
    "Used space threshold for the automatic shrinkage of the volume";
pub const METRIC_VOL_AUTOSIZE_GROW_THRESHOLD_NAME: &str =
    "netapp_volume_autosize_grow_threshold_percent";
pub const METRIC_VOL_AUTOSIZE_GROW_THRESHOLD_HELP: &str =
    "Used space threshold for the automatic growth of the volume";
pub const METRIC_VOL_AUTOSIZE_MODE_NAME: &str = "netapp_volume_autosize_mode_info";
pub const METRIC_VOL_AUTOSIZE_MODE_HELP: &str = "Autosize mode for the volume";
