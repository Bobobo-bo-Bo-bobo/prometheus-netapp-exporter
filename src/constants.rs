pub const NAME: &str = "prometheus-netapp-exporter";
pub const VERSION: &str = "0.1.1-20220205";
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
pub const API_QUOTAS: &str = "/api/storage/quota/reports";
pub const API_CHASSIS: &str = "/api/cluster/chassis";
pub const API_JOBS: &str = "/api/cluster/jobs";

pub const TARGET_AGGREGATES: u64 = 0x0000000000000001;
pub const TARGET_VOLUMES: u64 = 0x0000000000000002;
pub const TARGET_QUOTAS: u64 = 0x0000000000000004;
pub const TARGET_CHASSIS: u64 = 0x0000000000000008;
pub const TARGET_JOBS: u64 = 0x0000000000000010;

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
pub const METRIC_VOL_IS_OBJECT_STORE_NAME: &str = "netapp_volume_is_object_store_info";
pub const METRIC_VOL_IS_OBJECT_STORE_HELP: &str =
    "Specifies whether the volume is provisioned for an object store server";
pub const METRIC_VOL_NUMBER_OF_AGGREGATES_NAME: &str = "netapp_volume_number_of_aggregates";
pub const METRIC_VOL_NUMBER_OF_AGGREGATES_HELP: &str = "Aggregate hosting the volume";
pub const METRIC_VOL_FLEX_CACHE_ENDPOINT_TYPE_NAME: &str = "netapp_volume_flex_cache_info";
pub const METRIC_VOL_FLEX_CACHE_ENDPOINT_TYPE_HELP: &str = "FlexCache endpoint type";
pub const METRIC_VOL_TYPE_NAME: &str = "netapp_volume_type_info";
pub const METRIC_VOL_TYPE_HELP: &str = "Type of the volume";
pub const METRIC_VOL_CLOUD_RETRIEVAL_POLICY_NAME: &str = "netapp_volume_cloud_retrieval_info";
pub const METRIC_VOL_CLOUD_RETRIEVAL_POLICY_HELP: &str = "Cloud retrieval policy for the volume";
pub const METRIC_VOL_QUOTA_STATE_NAME: &str = "netapp_volume_quota_state_info";
pub const METRIC_VOL_QUOTA_STATE_HELP: &str = "Quota state of the volume";
pub const METRIC_VOL_EFFICIENCY_COMPRESSION_NAME: &str =
    "netapp_volume_efficiency_compression_info";
pub const METRIC_VOL_EFFICIENCY_COMPRESSION_HELP: &str = "Compression state of the volume";
pub const METRIC_VOL_EFFICIENCY_COMPACTION_NAME: &str = "netapp_volume_efficiency_compaction_info";
pub const METRIC_VOL_EFFICIENCY_COMPACTION_HELP: &str = "Compaction state of the volume";
pub const METRIC_VOL_EFFICIENCY_DEDUPE_NAME: &str = "netapp_volume_efficiency_dedupe_info";
pub const METRIC_VOL_EFFICIENCY_DEDUPE_HELP: &str = "Deduplication state of the volume";
pub const METRIC_VOL_EFFICIENCY_CROSS_VOLUME_DEDUPE_NAME: &str =
    "netapp_volume_efficiency_cross_volume_dedupe_info";
pub const METRIC_VOL_EFFICIENCY_CROSS_VOLUME_DEDUPE_HELP: &str =
    "Cross volume deduplication state of the volume";
pub const METRIC_VOL_METRIC_SAMPLE_DURATION_NAME: &str =
    "netapp_volume_metric_sample_duration_seconds";
pub const METRIC_VOL_METRIC_SAMPLE_DURATION_HELP: &str =
    "The duration over which the sample are calculated";
pub const METRIC_VOL_METRIC_IOPS_READ_NAME: &str = "netapp_volume_metric_iops_read_iops_per_second";
pub const METRIC_VOL_METRIC_IOPS_READ_HELP: &str =
    "The rate of I/O operations observed at the storage object for read operations";
pub const METRIC_VOL_METRIC_IOPS_WRITE_NAME: &str =
    "netapp_volume_metric_iops_write_iops_per_second";
pub const METRIC_VOL_METRIC_IOPS_WRITE_HELP: &str =
    "The rate of I/O operations observed at the storage object for write operations";
pub const METRIC_VOL_METRIC_IOPS_OTHER_NAME: &str =
    "netapp_volume_metric_iops_other_iops_per_second";
pub const METRIC_VOL_METRIC_IOPS_OTHER_HELP: &str =
    "The rate of I/O operations observed at the storage object for other operations";
pub const METRIC_VOL_METRIC_IOPS_TOTAL_NAME: &str =
    "netapp_volume_metric_iops_total_iops_per_second";
pub const METRIC_VOL_METRIC_IOPS_TOTAL_HELP: &str =
    "The rate of I/O operations observed at the storage object for all operations";
pub const METRIC_VOL_METRIC_LATENCY_READ_NAME: &str = "netapp_volume_metric_latency_read_seconds";
pub const METRIC_VOL_METRIC_LATENCY_READ_HELP: &str =
    "The round trip latency in microseconds observed at the storage object for read operations";
pub const METRIC_VOL_METRIC_LATENCY_WRITE_NAME: &str = "netapp_volume_metric_latency_write_seconds";
pub const METRIC_VOL_METRIC_LATENCY_WRITE_HELP: &str =
    "The round trip latency in microseconds observed at the storage object for write operations";
pub const METRIC_VOL_METRIC_LATENCY_OTHER_NAME: &str = "netapp_volume_metric_latency_other_seconds";
pub const METRIC_VOL_METRIC_LATENCY_OTHER_HELP: &str =
    "The round trip latency in microseconds observed at the storage object for other operations";
pub const METRIC_VOL_METRIC_LATENCY_TOTAL_NAME: &str = "netapp_volume_metric_latency_total_seconds";
pub const METRIC_VOL_METRIC_LATENCY_TOTAL_HELP: &str =
    "The round trip latency in microseconds observed at the storage object for all operations";
pub const METRIC_VOL_METRIC_THROUGHPUT_READ_NAME: &str =
    "netapp_volume_metric_throughput_read_bytes_per_second";
pub const METRIC_VOL_METRIC_THROUGHPUT_READ_HELP: &str =
    "Performance metric for read I/O operations in bytes per second";
pub const METRIC_VOL_METRIC_THROUGHPUT_WRITE_NAME: &str =
    "netapp_volume_metric_throughput_write_bytes_per_second";
pub const METRIC_VOL_METRIC_THROUGHPUT_WRITE_HELP: &str =
    "Performance metric for write I/O operations in bytes per second";
pub const METRIC_VOL_METRIC_THROUGHPUT_OTHER_NAME: &str =
    "netapp_volume_metric_throughput_other_bytes_per_second";
pub const METRIC_VOL_METRIC_THROUGHPUT_OTHER_HELP: &str =
    "Performance metric for other I/O operations in bytes per second";
pub const METRIC_VOL_METRIC_THROUGHPUT_TOTAL_NAME: &str =
    "netapp_volume_metric_throughput_total_bytes_per_second";
pub const METRIC_VOL_METRIC_THROUGHPUT_TOTAL_HELP: &str =
    "Performance metric for all I/O operations in bytes per second";
pub const METRIC_VOL_METRIC_CLOUD_IOPS_READ_NAME: &str =
    "netapp_volume_metric_cloud_iops_read_iops_per_second";
pub const METRIC_VOL_METRIC_CLOUD_IOPS_READ_HELP: &str =
    "The rate of I/O operations observed at the cloud storage object for read operations";
pub const METRIC_VOL_METRIC_CLOUD_IOPS_WRITE_NAME: &str =
    "netapp_volume_metric_cloud_iops_write_iops_per_second";
pub const METRIC_VOL_METRIC_CLOUD_IOPS_WRITE_HELP: &str =
    "The rate of I/O operations observed at the cloud storage object for write operations";
pub const METRIC_VOL_METRIC_CLOUD_IOPS_OTHER_NAME: &str =
    "netapp_volume_metric_cloud_iops_other_iops_per_second";
pub const METRIC_VOL_METRIC_CLOUD_IOPS_OTHER_HELP: &str =
    "The rate of I/O operations observed at the cloud storage object for other operations";
pub const METRIC_VOL_METRIC_CLOUD_IOPS_TOTAL_NAME: &str =
    "netapp_volume_metric_cloud_iops_total_iops_per_second";
pub const METRIC_VOL_METRIC_CLOUD_IOPS_TOTAL_HELP: &str =
    "The rate of I/O operations observed at the cloud storage object for all operations";
pub const METRIC_VOL_METRIC_CLOUD_LATENCY_READ_NAME: &str =
    "netapp_volume_metric_cloud_latency_read_seconds";
pub const METRIC_VOL_METRIC_CLOUD_LATENCY_READ_HELP: &str =
    "The round trip latency in microseconds observed at the cloud storage object for read operations";
pub const METRIC_VOL_METRIC_CLOUD_LATENCY_WRITE_NAME: &str =
    "netapp_volume_metric_cloud_latency_write_seconds";
pub const METRIC_VOL_METRIC_CLOUD_LATENCY_WRITE_HELP: &str =
    "The round trip latency in microseconds observed at the cloud storage object for write operations";
pub const METRIC_VOL_METRIC_CLOUD_LATENCY_OTHER_NAME: &str =
    "netapp_volume_metric_cloud_latency_other_seconds";
pub const METRIC_VOL_METRIC_CLOUD_LATENCY_OTHER_HELP: &str =
    "The round trip latency in microseconds observed at the cloud storage object for other operations";
pub const METRIC_VOL_METRIC_CLOUD_LATENCY_TOTAL_NAME: &str =
    "netapp_volume_metric_cloud_latency_total_seconds";
pub const METRIC_VOL_METRIC_CLOUD_LATENCY_TOTAL_HELP: &str =
    "The round trip latency in microseconds observed at the cloud storage object for all operations";
pub const METRIC_VOL_METRIC_CLOUD_SAMPLE_DURATION_NAME: &str =
    "netapp_volume_metric_sample_cloud_storage_duration_seconds";
pub const METRIC_VOL_METRIC_CLOUD_SAMPLE_DURATION_HELP: &str =
    "The duration over which the sample are calculated";
pub const METRIC_VOL_METRIC_FLEXCACHE_SAMPLE_DURATION_NAME: &str =
    "netapp_volume_metric_flexcache_sample_duration_seconds";
pub const METRIC_VOL_METRIC_FLEXCACHE_SAMPLE_DURATION_HELP: &str =
    "The duration over which the sample are calculated";
pub const METRIC_VOL_METRIC_FLEXCACHE_CACHE_MISS_PERCENT_NAME: &str =
    "netapp_volume_metric_flexcache_cache_miss_percent";
pub const METRIC_VOL_METRIC_FLEXCACHE_CACHE_MISS_PERCENT_HELP: &str = "Cache miss percentage";
pub const METRIC_VOL_ACCESS_TIME_ENABLED_NAME: &str = "netapp_volume_metric_access_time_enabled";
pub const METRIC_VOL_ACCESS_TIME_ENABLED_HELP: &str =
    "Indicates whether or not access time updates are enabled on the volume";
pub const METRIC_VOL_QUEUE_FOR_ENCRYPTION_NAME: &str = "netapp_volume_queued_for_encryption";
pub const METRIC_VOL_QUEUE_FOR_ENCRYPTION_HELP: &str =
    "Specifies whether the volume is queued for encryption";
pub const METRIC_VOL_SNAPLOCK_APPEND_MODE_ENABLED_NAME: &str =
    "netapp_volume_snaplock_append_mode_enabled";
pub const METRIC_VOL_SNAPLOCK_APPEND_MODE_ENABLED_HELP: &str =
    "Specifies if the volume append mode is enabled or disabled";
pub const METRIC_VOL_SNAPLOCK_LITIGATION_COUNT_NAME: &str =
    "netapp_volume_snaplock_litigation_count";
pub const METRIC_VOL_SNAPLOCK_LITIGATION_COUNT_HELP: &str =
    "Litigation count indicates the number of active legal-holds on the volume";
pub const METRIC_VOL_SNAPLOCK_UNSPECIFIED_RETENTION_FILE_COUNT_NAME: &str =
    "netapp_volume_snaplock_unspecified_retention_files";
pub const METRIC_VOL_SNAPLOCK_UNSPECIFIED_RETENTION_FILE_COUNT_HELP: &str =
    "Indicates the number of files with an unspecified retention time in the volume";
pub const METRIC_VOL_SNAPLOCK_IS_AUDIT_LOG_NAME: &str = "netapp_volume_snaplock_is_audit_log";
pub const METRIC_VOL_SNAPLOCK_IS_AUDIT_LOG_HELP: &str =
    "Indicates if this volume has been configured as SnapLock audit log volume for the SVM";
pub const METRIC_VOL_SNAPLOCK_PRIVILEGED_DELETE_NAME: &str =
    "netapp_volume_snaplock_privileged_delete";
pub const METRIC_VOL_SNAPLOCK_PRIVILEGED_DELETE_HELP: &str =
    "Specifies the privileged-delete attribute of a SnapLock volume";
pub const METRIC_VOL_SNAPLOCK_TYPE_NAME: &str = "netapp_volume_snaplock_type_info";
pub const METRIC_VOL_SNAPLOCK_TYPE_HELP: &str = "The SnapLock type of the volume";
pub const METRIC_VOL_MOVEMENT_PERCENT_COMPLETE_NAME: &str =
    "netapp_volume_movemet_complete_percent";
pub const METRIC_VOL_MOVEMENT_PERCENT_COMPLETE_HELP: &str =
    "Completion percentage of volume movement";
pub const METRIC_VOL_MOVEMENT_CUTOVER_WINDOW_NAME: &str =
    "netapp_volume_movement_cutover_window_seconds";
pub const METRIC_VOL_MOVEMENT_CUTOVER_WINDOW_HELP: &str = "Time window in seconds for cutover";
pub const METRIC_VOL_MOVEMENT_TIERING_POLICY_NAME: &str =
    "netapp_volume_movement_tiering_policy_info";
pub const METRIC_VOL_MOVEMENT_TIERING_POLICY_HELP: &str = "Tiering policy for FabricPool";
pub const METRIC_VOL_MOVEMENT_STATE_NAME: &str = "netapp_volume_movement_state_info";
pub const METRIC_VOL_MOVEMENT_STATE_HELP: &str = "State of volume move operation";
pub const METRIC_VOL_STYLE_NAME: &str = "netapp_volume_style";
pub const METRIC_VOL_STYLE_HELP: &str = "The style of the volume";
pub const METRIC_VOL_ENCRYPTION_TYPE_NAME: &str = "netapp_volume_encryption_type_info";
pub const METRIC_VOL_ENCRYPTION_TYPE_HELP: &str = "Volume encryption type";
pub const METRIC_VOL_ENCRYPTION_STATE_NAME: &str = "netapp_volume_encryption_state_info";
pub const METRIC_VOL_ENCRYPTION_STATE_HELP: &str = "Volume encryption state";
pub const METRIC_VOL_ENCRYPTION_ENABLED_NAME: &str = "neapp_volume_encryption_enabled";
pub const METRIC_VOL_ENCRYPTION_ENABLED_HELP: &str = "Volume encryption state";
pub const METRIC_VOL_TIERING_POLICY_NAME: &str = "netapp_volume_tiering_policy_info";
pub const METRIC_VOL_TIERING_POLICY_HELP: &str = "Policy that determines whether the user data blocks of a volume in a FabricPool will be tiered to the cloud store when they become cold";
pub const METRIC_VOL_TIERING_SUPPORTED_NAME: &str = "netapp_volume_tiering_supported";
pub const METRIC_VOL_TIERING_SUPPORTED_HELP: &str = "Whether or not FabricPools are selected when provisioning a FlexGroup without specifying aggregates.name or aggregates.uuid";
pub const METRIC_VOL_TIERING_MIN_COOLING_DAYS_NAME: &str = "netapp_volume_tiering_min_cooling_days";
pub const METRIC_VOL_TIERING_MIN_COOLING_DAYS_HELP: &str = "Minimum number of days that user data blocks of the volume must be cooled before they can be considered cold and tiered out to the cloud tier";
pub const METRIC_VOL_SPACE_BLOCKSTORAGE_INACTIVE_USER_DATA_NAME: &str =
    "netapp_volume_space_blockstorage_inactive_user_data_bytes";
pub const METRIC_VOL_SPACE_BLOCKSTORAGE_INACTIVE_USER_DATA_HELP: &str = "The size that is physically used in the block storage of the volume and has a cold temperature";
pub const METRIC_VOL_SPACE_OVER_PROVISIONED_NAME: &str =
    "netapp_volume_space_over_provisioned_bytes";
pub const METRIC_VOL_SPACE_OVER_PROVISIONED_HELP: &str =
    "The amount of space not available for this volume in the aggregate, in bytes";
pub const METRIC_VOL_SPACE_PERFORMANCE_TIER_FOOTPRINT_NAME: &str =
    "netapp_volume_space_performance_tier_footprint_bytes";
pub const METRIC_VOL_SPACE_PERFORMANCE_TIER_FOOTPRINT_HELP: &str =
    "Space used by the performance tier for this volume in the FabricPool aggregate, in bytes";
pub const METRIC_VOL_SPACE_FOOTPRINT_NAME: &str = "netapp_volume_space_footprint_bytes";
pub const METRIC_VOL_SPACE_FOOTPRINT_HELP: &str = "Data used for this volume in the aggregate";
pub const METRIC_VOL_SPACE_CAPACITY_TIER_FOOTPRINT_NAME: &str =
    "netapp_volume_space_capacity_tier_footprint_bytes";
pub const METRIC_VOL_SPACE_CAPACITY_TIER_FOOTPRINT_HELP: &str =
    "Space used by capacity tier for this volume in the FabricPool aggregate";
pub const METRIC_VOL_SPACE_TOTAL_FOOTPRINT_NAME: &str = "netapp_volume_space_total_footprint_bytes";
pub const METRIC_VOL_SPACE_TOTAL_FOOTPRINT_HELP: &str =
    "Data and metadata used for this volume in the aggregate";
pub const METRIC_VOL_SPACE_SIZE_NAME: &str = "netapp_volume_space_size_bytes";
pub const METRIC_VOL_SPACE_SIZE_HELP: &str = "Total provisioned size";
pub const METRIC_VOL_SPACE_LOGICAL_SPACE_REPORTING_NAME: &str =
    "netapp_volume_space_logical_space_reporting_enabled";
pub const METRIC_VOL_SPACE_LOGICAL_SPACE_REPORTING_HELP: &str =
    "Whether space reporting on the volume is done along with storage efficiency";
pub const METRIC_VOL_SPACE_LOGICAL_SPACE_ENFORCEMENT_NAME: &str =
    "netapp_volume_space_logical_space_enforcement_enabled";
pub const METRIC_VOL_SPACE_LOGICAL_SPACE_ENFORCEMENT_HELP: &str =
    "Whether space accounting for operations on the volume is done along with storage efficiency";
pub const METRIC_VOL_SPACE_LOGICAL_SPACE_USED_BY_AFS_NAME: &str =
    "netapp_volume_space_logical_space_used_by_afs_bytes";
pub const METRIC_VOL_SPACE_LOGICAL_SPACE_USED_BY_AFS_HELP: &str = "The virtual space used by AFS alone (includes volume reserves) and along with storage efficiency";
pub const METRIC_VOL_SPACE_LOGICAL_SPACE_AVAILABLE_NAME: &str =
    "netapp_volume_space_logical_space_available_bytes";
pub const METRIC_VOL_SPACE_LOGICAL_SPACE_AVAILABLE_HELP: &str =
    "The amount of space available in this volume with storage efficiency space considered used";
pub const METRIC_VOL_SPACE_USED_NAME: &str = "netapp_volume_space_used_bytes";
pub const METRIC_VOL_SPACE_USED_HELP: &str =
    "The virtual space used (includes volume reserves) before storage efficiency";
pub const METRIC_VOL_SPACE_SNAPSHOT_AUTODELETE_ENABLED_NAME: &str =
    "netapp_volume_space_snapshot_autodalete_enabled";
pub const METRIC_VOL_SPACE_SNAPSHOT_AUTODELETE_ENABLED_HELP: &str =
    "Whether Snapshot copy autodelete is currently enabled on this volume";
pub const METRIC_VOL_SPACE_SNAPSHOT_USED_NAME: &str = "netapp_volume_space_snapshot_used_bytes";
pub const METRIC_VOL_SPACE_SNAPSHOT_USED_HELP: &str =
    "The total space used by Snapshot copies in the volume";
pub const METRIC_VOL_SPACE_SNAPSHOT_RESERVE_PERCENT_NAME: &str =
    "netapp_volume_space_snapshot_reserve_percent";
pub const METRIC_VOL_SPACE_SNAPSHOT_RESERVE_PERCENT_HELPE: &str =
    "The space that has been set aside as a reserve for Snapshot copy usage";
pub const METRIC_VOL_SPACE_METADATA_NAME: &str = "netapp_volume_space_metadata_bytes";
pub const METRIC_VOL_SPACE_METADATA_HELP: &str =
    "Space used by the volume metadata in the aggregate";
pub const METRIC_VOL_SPACE_AVAILABLE_NAME: &str = "netapp_volume_space_available_bytes";
pub const METRIC_VOL_SPACE_AVAILABLE_HELP: &str = "The available space";
pub const METRIC_VOL_SPACE_LOCAL_TIER_FOOTPRINT_NAME: &str =
    "netapp_volume_space_local_tier_footprint_bytes";
pub const METRIC_VOL_SPACE_LOCAL_TIER_FOOTPRINT_HELP: &str =
    "Space used by the local tier for this volume in the aggregate";
pub const METRIC_VOL_ANALYTICS_SCAN_PROGRESS_NAME: &str =
    "netapp_volume_analytics_scan_progress_percent";
pub const METRIC_VOL_ANALYTICS_SCAN_PROGRESS_HELP: &str = "Percentage of files in the volume that the file system analytics initialization scan has processed";
pub const METRIC_VOL_ANALYTICS_SUPPORTED_NAME: &str = "netapp_volume_analytics_supported";
pub const METRIC_VOL_ANALYTICS_SUPPORTED_HELP: &str =
    "Whether or not file system analytics is supported on the volume";
pub const METRIC_VOL_ANALYTICS_STATE_NAME: &str = "netapp_volume_analytics_state_info";
pub const METRIC_VOL_ANALYTICS_STATE_HELP: &str = "File system analytics state of the volume";
pub const METRIC_VOL_GUARANTEE_TYPE_NAME: &str = "netapp_volume_guarantee_type_info";
pub const METRIC_VOL_GUARANTEE_TYPE_HELP: &str =
    "The type of space guarantee of this volume in the aggregate";
pub const METRIC_VOL_GUARANTEE_HONORED_NAME: &str = "netapp_volume_guarantee_honored";
pub const METRIC_VOL_GUARANTEE_HONORED_HELP: &str =
    "Wheter the space guarantee of this volume honored in the aggregate";
pub const METRIC_VOL_IS_SVM_ROOT_NAME: &str = "netapp_volume_is_svm_root";
pub const METRIC_VOL_IS_SVM_ROOT_HELP: &str =
    "Whether the volume is a root volume of the SVM it belongs to";
pub const METRIC_VOL_USE_MIRRORED_AGGREGATES_NAME: &str = "netapp_volume_use_mirrored_aggregates";
pub const METRIC_VOL_USE_MIRRORED_AGGREGATES_HELP: &str = "Specifies whether mirrored aggregates are selected when provisioning a FlexGroup without specifying aggregates.name or aggregates.uuid";
pub const METRIC_VOL_SNAPMIRROR_PROTECTED_NAME: &str = "netapp_volume_snapmirror_protected";
pub const METRIC_VOL_SNAPMIRROR_PROTECTED_HELP: &str = "Specifies whether a volume is a SnapMirror source volume, using SnapMirror to protect its data";

// Quota metrics
pub const METRIC_TREE_QUOTA_SPACE_HARD_LIMIT_NAME: &str =
    "netapp_tree_quota_space_hard_limit_bytes";
pub const METRIC_TREE_QUOTA_SPACE_HARD_LIMIT_HELP: &str = "Space hard limit in bytes";
pub const METRIC_TREE_QUOTA_SPACE_HARD_LIMIT_PERCENT_NAME: &str =
    "netapp_tree_quota_space_hard_limit_percent";
pub const METRIC_TREE_QUOTA_SPACE_HARD_LIMIT_PERCENT_HELP: &str =
    "Total space used as a percentage of space hard limit";
pub const METRIC_TREE_QUOTA_SPACE_SOFT_LIMIT_NAME: &str =
    "netapp_tree_quota_space_soft_limit_bytes";
pub const METRIC_TREE_QUOTA_SPACE_SOFT_LIMIT_HELP: &str = "Space soft limit in bytes";
pub const METRIC_TREE_QUOTA_SPACE_SOFT_LIMIT_PERCENT_NAME: &str =
    "netapp_tree_quota_space_soft_limit_percent";
pub const METRIC_TREE_QUOTA_SPACE_SOFT_LIMIT_PERCENT_HELP: &str =
    "Total space used as a percentage of space soft limit";
pub const METRIC_TREE_QUOTA_SPACE_USED_NAME: &str = "netapp_tree_quota_space_used_bytes";
pub const METRIC_TREE_QUOTA_SPACE_USED_HELP: &str = "Total space used";
pub const METRIC_TREE_QUOTA_FILES_HARD_LIMIT_NAME: &str = "netapp_tree_quota_files_hard_limit";
pub const METRIC_TREE_QUOTA_FILES_HARD_LIMIT_HELP: &str = "Files hard limit";
pub const METRIC_TREE_QUOTA_FILES_HARD_LIMIT_PERCENT_NAME: &str =
    "netapp_tree_quota_files_hard_limit_percent";
pub const METRIC_TREE_QUOTA_FILES_HARD_LIMIT_PERCENT_HELP: &str =
    "Total files used as a percentage of files hard limit";
pub const METRIC_TREE_QUOTA_FILES_SOFT_LIMIT_NAME: &str = "netapp_tree_quota_files_soft_limit";
pub const METRIC_TREE_QUOTA_FILES_SOFT_LIMIT_HELP: &str = "Files soft limit";
pub const METRIC_TREE_QUOTA_FILES_SOFT_LIMIT_PERCENT_NAME: &str =
    "netapp_tree_quota_files_soft_limit_percent";
pub const METRIC_TREE_QUOTA_FILES_SOFT_LIMIT_PERCENT_HELP: &str =
    "Total files used as a percentage of files soft limit";
pub const METRIC_TREE_QUOTA_FILES_USED_NAME: &str = "netapp_tree_quota_files_used";
pub const METRIC_TREE_QUOTA_FILES_USED_HELP: &str = "Total files used";
pub const METRIC_GROUP_QUOTA_SPACE_HARD_LIMIT_NAME: &str =
    "netapp_group_quota_space_hard_limit_bytes";
pub const METRIC_GROUP_QUOTA_SPACE_HARD_LIMIT_HELP: &str = "Space hard limit in bytes";
pub const METRIC_GROUP_QUOTA_SPACE_HARD_LIMIT_PERCENT_NAME: &str =
    "netapp_group_quota_space_hard_limit_percent";
pub const METRIC_GROUP_QUOTA_SPACE_HARD_LIMIT_PERCENT_HELP: &str =
    "Total space used as a percentage of space hard limit";
pub const METRIC_GROUP_QUOTA_SPACE_SOFT_LIMIT_NAME: &str =
    "netapp_group_quota_space_soft_limit_bytes";
pub const METRIC_GROUP_QUOTA_SPACE_SOFT_LIMIT_HELP: &str = "Space soft limit in bytes";
pub const METRIC_GROUP_QUOTA_SPACE_SOFT_LIMIT_PERCENT_NAME: &str =
    "netapp_group_quota_space_soft_limit_percent";
pub const METRIC_GROUP_QUOTA_SPACE_SOFT_LIMIT_PERCENT_HELP: &str =
    "Total space used as a percentage of space soft limit";
pub const METRIC_GROUP_QUOTA_SPACE_USED_NAME: &str = "netapp_group_quota_space_used_bytes";
pub const METRIC_GROUP_QUOTA_SPACE_USED_HELP: &str = "Total space used";
pub const METRIC_GROUP_QUOTA_FILES_HARD_LIMIT_NAME: &str = "netapp_group_quota_files_hard_limit";
pub const METRIC_GROUP_QUOTA_FILES_HARD_LIMIT_HELP: &str = "Files hard limit";
pub const METRIC_GROUP_QUOTA_FILES_HARD_LIMIT_PERCENT_NAME: &str =
    "netapp_group_quota_files_hard_limit_percent";
pub const METRIC_GROUP_QUOTA_FILES_HARD_LIMIT_PERCENT_HELP: &str =
    "Total files used as a percentage of files hard limit";
pub const METRIC_GROUP_QUOTA_FILES_SOFT_LIMIT_NAME: &str = "netapp_group_quota_files_soft_limit";
pub const METRIC_GROUP_QUOTA_FILES_SOFT_LIMIT_HELP: &str = "Files soft limit";
pub const METRIC_GROUP_QUOTA_FILES_SOFT_LIMIT_PERCENT_NAME: &str =
    "netapp_group_quota_files_soft_limit_percent";
pub const METRIC_GROUP_QUOTA_FILES_SOFT_LIMIT_PERCENT_HELP: &str =
    "Total files used as a percentage of files soft limit";
pub const METRIC_GROUP_QUOTA_FILES_USED_NAME: &str = "netapp_group_quota_files_used";
pub const METRIC_GROUP_QUOTA_FILES_USED_HELP: &str = "Total files used";
pub const METRIC_USER_QUOTA_SPACE_HARD_LIMIT_NAME: &str =
    "netapp_user_quota_space_hard_limit_bytes";
pub const METRIC_USER_QUOTA_SPACE_HARD_LIMIT_HELP: &str = "Space hard limit in bytes";
pub const METRIC_USER_QUOTA_SPACE_HARD_LIMIT_PERCENT_NAME: &str =
    "netapp_user_quota_space_hard_limit_percent";
pub const METRIC_USER_QUOTA_SPACE_HARD_LIMIT_PERCENT_HELP: &str =
    "Total space used as a percentage of space hard limit";
pub const METRIC_USER_QUOTA_SPACE_SOFT_LIMIT_NAME: &str =
    "netapp_user_quota_space_soft_limit_bytes";
pub const METRIC_USER_QUOTA_SPACE_SOFT_LIMIT_HELP: &str = "Space soft limit in bytes";
pub const METRIC_USER_QUOTA_SPACE_SOFT_LIMIT_PERCENT_NAME: &str =
    "netapp_user_quota_space_soft_limit_percent";
pub const METRIC_USER_QUOTA_SPACE_SOFT_LIMIT_PERCENT_HELP: &str =
    "Total space used as a percentage of space soft limit";
pub const METRIC_USER_QUOTA_SPACE_USED_NAME: &str = "netapp_user_quota_space_used_bytes";
pub const METRIC_USER_QUOTA_SPACE_USED_HELP: &str = "Total space used";
pub const METRIC_USER_QUOTA_FILES_HARD_LIMIT_NAME: &str = "netapp_user_quota_files_hard_limit";
pub const METRIC_USER_QUOTA_FILES_HARD_LIMIT_HELP: &str = "Files hard limit";
pub const METRIC_USER_QUOTA_FILES_HARD_LIMIT_PERCENT_NAME: &str =
    "netapp_user_quota_files_hard_limit_percent";
pub const METRIC_USER_QUOTA_FILES_HARD_LIMIT_PERCENT_HELP: &str =
    "Total files used as a percentage of files hard limit";
pub const METRIC_USER_QUOTA_FILES_SOFT_LIMIT_NAME: &str = "netapp_user_quota_files_soft_limit";
pub const METRIC_USER_QUOTA_FILES_SOFT_LIMIT_HELP: &str = "Files soft limit";
pub const METRIC_USER_QUOTA_FILES_SOFT_LIMIT_PERCENT_NAME: &str =
    "netapp_user_quota_files_soft_limit_percent";
pub const METRIC_USER_QUOTA_FILES_SOFT_LIMIT_PERCENT_HELP: &str =
    "Total files used as a percentage of files soft limit";
pub const METRIC_USER_QUOTA_FILES_USED_NAME: &str = "netapp_user_quota_files_used";
pub const METRIC_USER_QUOTA_FILES_USED_HELP: &str = "Total files used";

// Cluster chassis metrics
pub const METRIC_CHASSIS_STATE_NAME: &str = "netapp_cluster_chassis_state_info";
pub const METRIC_CHASSIS_STATE_HELP: &str = "State of chassis";
pub const METRIC_CHASSIS_SHELVES_NAME: &str = "netapp_cluster_shassis_shelves";
pub const METRIC_CHASSIS_SHELVES_HELP: &str = "Number of shelves in the chassis";
pub const METRIC_CHASSIS_NODES_NAME: &str = "netapp_cluster_shassis_nodes";
pub const METRIC_CHASSIS_NODES_HELP: &str = "Number of nodes in the chassis";
pub const METRIC_CHASSIS_FRU_STATE_NAME: &str = "netapp_cluster_chassis_fru_state_info";
pub const METRIC_CHASSIS_FRU_STATE_HELP: &str = "State of FRU in chassis";
pub const METRIC_CHASSIS_USB_SUPPORTED_NAME: &str = "netapp_cluster_chassis_usb_supported";
pub const METRIC_CHASSIS_USB_SUPPORTED_HELP: &str = "Chassis USB ports are supported";
pub const METRIC_CHASSIS_USB_ENABLED_NAME: &str = "netapp_cluster_chassis_usb_enabled";
pub const METRIC_CHASSIS_USB_ENABLED_HELP: &str = "Chassis USB ports are enabled";
pub const METRIC_CHASSIS_USB_CONNECTED_STATE_NAME: &str = "netapp_cluster_chassis_usb_port_info";
pub const METRIC_CHASSIS_USB_CONNECTED_STATE_HELP: &str =
    "Number of connected or disconnected USB ports";

// Cluster job metrics
pub const METRIC_JOBS_STATE_NAME: &str = "netapp_cluster_job_state";
pub const METRIC_JOBS_STATE_HELP: &str = "The states of jobs on the cluster";
