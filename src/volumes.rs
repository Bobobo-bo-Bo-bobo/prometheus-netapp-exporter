use crate::config;
use crate::constants;
use crate::exporter;
use crate::http;
use crate::storage_metrics;

use log::{debug, error, warn};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeList {
    pub records: Vec<Volume>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Volume {
    pub name: String,
    // Fields are not set if the volume is on another node of the MetroCluster
    pub autosize: Option<VolumeAutoSize>,
    pub error_state: Option<VolumeErrorState>,
    pub is_object_store: Option<bool>,
    pub files: Option<VolumeFiles>,
    pub state: Option<String>,
    pub aggregates: Option<Vec<AggregateList>>,
    pub flexcache_endpoint_type: Option<String>,
    #[serde(rename = "type")]
    pub vol_type: Option<String>,
    pub cloud_retrieval_policy: Option<String>,
    pub quota: Option<VolumeQuota>,
    pub efficiency: Option<VolumeEfficiency>,
    pub metric: Option<storage_metrics::StorageMetric>,
    pub access_time_enabled: Option<bool>,
    pub queue_for_encryption: Option<bool>,
    pub snaplock: Option<VolumeSnaplock>,
    pub movement: Option<VolumeMovement>,
    pub style: Option<String>,
    pub encryption: Option<VolumeEncryption>,
    pub tiering: Option<VolumeTiering>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeTiering {
    pub policy: String,
    pub supported: Option<bool>,
    pub min_cooling_days: Option<i64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeEncryption {
    pub status: Option<VolumeEncryptionStatus>,
    #[serde(rename = "type")]
    pub enc_type: Option<String>,
    pub state: Option<String>,
    pub enabled: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeEncryptionStatus {
    pub message: Option<String>,
    pub key_id: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeMovement {
    pub percent_complete: Option<i64>,
    pub cutover_window: Option<i64>,
    pub tiering_policy: Option<String>,
    pub state: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeSnaplock {
    pub append_mode_enabled: Option<bool>,
    pub litigation_count: Option<i64>,
    pub unspecified_retention_file_count: Option<i64>,
    pub is_audit_log: Option<bool>,
    pub privileged_delete: Option<String>,
    #[serde(rename = "type")]
    pub snaplock_type: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeEfficiency {
    pub compression: Option<String>,
    pub compaction: Option<String>,
    pub dedupe: Option<String>,
    pub cross_volume_dedupe: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeQuota {
    pub state: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AggregateList {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeAutoSize {
    pub minimum: i64,
    pub shrink_threshold: i64,
    pub maximum: i64,
    pub mode: String,
    pub grow_threshold: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeErrorState {
    pub has_bad_blocks: bool,
    pub is_inconsistent: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VolumeFiles {
    pub maximum: i64,
    pub used: i64,
}

pub fn update_volumes(
    filer: &config::NetAppConfiguration,
    client: &mut reqwest::blocking::Client,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://{}{}?fields=**",
        filer.address,
        constants::API_VOLUMES
    );
    let raw_vols = match http::get(client, &url, &filer.user, &filer.password) {
        Ok(v) => v,
        Err(e) => {
            bail!("Request for volumes on {} failed - {}", filer.name, e);
        }
    };

    let vols: VolumeList = match serde_json::from_str(&raw_vols) {
        Ok(v) => v,
        Err(e) => bail!(
            "Can't decode response for volume information from {} as JSON - {}",
            filer.name,
            e
        ),
    };
    for vol in vols.records {
        if let Some(v) = vol.state {
            let mut error: i64 = 0;
            let mut mixed: i64 = 0;
            let mut online: i64 = 0;
            let mut offline: i64 = 0;
            let mut ok: bool = true;

            match v.as_str() {
                "error" => {
                    error = 1;
                }
                "mixed" => {
                    mixed = 1;
                }
                "online" => {
                    online = 1;
                }
                "offline" => {
                    offline = 1;
                }
                _ => {
                    ok = false;
                    error!(
                        "Invalid state {} for volume {} on {}",
                        v, vol.name, filer.name
                    );
                }
            };
            if ok {
                exporter::VOLUME_STATE
                    .with_label_values(&[&filer.name, &vol.name, "error"])
                    .set(error);
                exporter::VOLUME_STATE
                    .with_label_values(&[&filer.name, &vol.name, "mixed"])
                    .set(mixed);
                exporter::VOLUME_STATE
                    .with_label_values(&[&filer.name, &vol.name, "online"])
                    .set(online);
                exporter::VOLUME_STATE
                    .with_label_values(&[&filer.name, &vol.name, "offline"])
                    .set(offline);
            }
        } else {
            debug!("Volume {} not active on {}, skipping", vol.name, filer.name);
            continue;
        }

        if let Some(v) = vol.files {
            debug!(
                "Updating metrics for volume metric files maximum: {} {} -> {}",
                filer.name, vol.name, v.maximum
            );
            exporter::VOLUME_FILES_MAX
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.maximum);
            debug!(
                "Updating metrics for volume metric files used: {} {} -> {}",
                filer.name, vol.name, v.used
            );
            exporter::VOLUME_FILES_USED
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.used);
        }

        if let Some(v) = vol.error_state {
            if v.has_bad_blocks {
                exporter::VOLUME_ERROR_STATE
                    .with_label_values(&[&filer.name, &vol.name, "has_bad_blocks"])
                    .set(1)
            } else {
                exporter::VOLUME_ERROR_STATE
                    .with_label_values(&[&filer.name, &vol.name, "has_bad_blocks"])
                    .set(0)
            }
            if v.is_inconsistent {
                exporter::VOLUME_ERROR_STATE
                    .with_label_values(&[&filer.name, &vol.name, "is_inconsistent"])
                    .set(1)
            } else {
                exporter::VOLUME_ERROR_STATE
                    .with_label_values(&[&filer.name, &vol.name, "is_inconsistent"])
                    .set(0)
            }
        }

        if let Some(v) = vol.autosize {
            debug!(
                "Updating metrics for volume autosize minimum {} {} -> {}",
                filer.name, vol.name, v.minimum
            );
            exporter::VOLUME_AUTOSIZE_MIN
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.minimum);

            debug!(
                "Updating metrics for volume autosize maximum {} {} -> {}",
                filer.name, vol.name, v.maximum
            );
            exporter::VOLUME_AUTOSIZE_MAX
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.maximum);

            debug!(
                "Updating metrics for volume autosize mode {} {} -> {}",
                filer.name, vol.name, v.mode
            );
            let mut grow: i64 = 0;
            let mut grow_shrink: i64 = 0;
            let mut off: i64 = 0;
            let mut ok: bool = true;

            match v.mode.as_str() {
                "grow" => {
                    grow = 1;
                }
                "grow_shrink" => {
                    grow_shrink = 1;
                }
                "off" => {
                    off = 1;
                }
                _ => {
                    ok = false;
                    error!(
                        "Invalid autosize mode {} for volume {} on {}",
                        v.mode, vol.name, filer.name
                    );
                }
            };
            if ok {
                exporter::VOLUME_AUTOSIZE_MODE
                    .with_label_values(&[&filer.name, &vol.name, "grow"])
                    .set(grow);
                exporter::VOLUME_AUTOSIZE_MODE
                    .with_label_values(&[&filer.name, &vol.name, "grow_shrink"])
                    .set(grow_shrink);
                exporter::VOLUME_AUTOSIZE_MODE
                    .with_label_values(&[&filer.name, &vol.name, "off"])
                    .set(off);
            }

            debug!(
                "Updating metrics for volume autosize shrink_threshold {} {} -> {}",
                filer.name, vol.name, v.shrink_threshold
            );
            exporter::VOLUME_AUTOSIZE_SHRINK_THRESHOLD
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.shrink_threshold);

            debug!(
                "Updating metrics for volume autosize grow_threshold {} {} -> {}",
                filer.name, vol.name, v.grow_threshold
            );
            exporter::VOLUME_AUTOSIZE_GROW_THRESHOLD
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.grow_threshold);
        }

        if let Some(v) = vol.is_object_store {
            debug!(
                "Updating metrics for volume is_object_store {} {} -> {}",
                filer.name, vol.name, v
            );
            if v {
                exporter::VOLUME_IS_OBJECT_STORE
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(1);
            } else {
                exporter::VOLUME_IS_OBJECT_STORE
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(0);
            }
        }

        if let Some(v) = vol.aggregates {
            debug!(
                "Updating metrics for volume aggregates {} {} -> {}",
                filer.name,
                vol.name,
                v.len()
            );
            exporter::VOLUME_NUMBER_OF_AGGREGATES
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.len() as i64);
        }

        if let Some(v) = vol.flexcache_endpoint_type {
            let mut none: i64 = 0;
            let mut cache: i64 = 0;
            let mut origin: i64 = 0;
            let mut ok: bool = true;

            debug!(
                "Updating metrics for volume flexcache_endpoint_type {} {} -> {}",
                filer.name, vol.name, v
            );
            match v.as_str() {
                "none" => {
                    none = 1;
                }
                "cache" => {
                    cache = 1;
                }
                "origin" => {
                    origin = 1;
                }
                _ => {
                    ok = false;
                    error!(
                        "Invalid value for flexcache_endpoint_type {} for volume {} on {}",
                        v, vol.name, filer.name
                    );
                }
            };
            if ok {
                exporter::VOLUME_FLEX_CACHE_ENDPOINT_TYPE
                    .with_label_values(&[&filer.name, &vol.name, "none"])
                    .set(none);
                exporter::VOLUME_FLEX_CACHE_ENDPOINT_TYPE
                    .with_label_values(&[&filer.name, &vol.name, "cache"])
                    .set(cache);
                exporter::VOLUME_FLEX_CACHE_ENDPOINT_TYPE
                    .with_label_values(&[&filer.name, &vol.name, "origin"])
                    .set(origin);
            }
        }

        if let Some(v) = vol.vol_type {
            let mut rw: i64 = 0;
            let mut dp: i64 = 0;
            let mut ls: i64 = 0;
            let mut ok: bool = true;

            debug!(
                "Updating metrics for volume type {} {} -> {}",
                filer.name, vol.name, v
            );
            match v.as_str() {
                "rw" => {
                    rw = 1;
                }
                "db" => {
                    dp = 1;
                }
                "ls" => {
                    ls = 1;
                }
                _ => {
                    ok = false;
                    error!(
                        "Invalid value for volume type {} for volume {} on {}",
                        v, vol.name, filer.name
                    );
                }
            }
            if ok {
                exporter::VOLUME_TYPE
                    .with_label_values(&[&filer.name, &vol.name, "rw"])
                    .set(rw);
                exporter::VOLUME_TYPE
                    .with_label_values(&[&filer.name, &vol.name, "dp"])
                    .set(dp);
                exporter::VOLUME_TYPE
                    .with_label_values(&[&filer.name, &vol.name, "ls"])
                    .set(ls);
            }
        }

        if let Some(v) = vol.cloud_retrieval_policy {
            let mut default: i64 = 0;
            let mut on_read: i64 = 0;
            let mut never: i64 = 0;
            let mut promote: i64 = 0;
            let mut ok: bool = true;

            debug!(
                "Updating metrics for volume cloud_retrieval_policy {} {} -> {}",
                filer.name, vol.name, v
            );
            match v.as_str() {
                "default" => {
                    default = 1;
                }
                "on_read" => {
                    on_read = 1;
                }
                "never" => {
                    never = 1;
                }
                "promote" => {
                    promote = 1;
                }
                _ => {
                    ok = false;
                    error!(
                        "Invalid value {} for cloud_retrieval_policy for volume {} on {}",
                        v, vol.name, filer.name
                    );
                }
            };
            if ok {
                exporter::VOLUME_CLOUD_RETRIEVAL_POLICY
                    .with_label_values(&[&filer.name, &vol.name, "default"])
                    .set(default);
                exporter::VOLUME_CLOUD_RETRIEVAL_POLICY
                    .with_label_values(&[&filer.name, &vol.name, "on_read"])
                    .set(on_read);
                exporter::VOLUME_CLOUD_RETRIEVAL_POLICY
                    .with_label_values(&[&filer.name, &vol.name, "never"])
                    .set(never);
                exporter::VOLUME_CLOUD_RETRIEVAL_POLICY
                    .with_label_values(&[&filer.name, &vol.name, "promote"])
                    .set(promote);
            }
        }

        if let Some(v) = vol.quota {
            debug!(
                "Updating metrics for volume quota state {} {} -> {}",
                filer.name, vol.name, v.state
            );
            let mut corrupt: i64 = 0;
            let mut initializing: i64 = 0;
            let mut mixed: i64 = 0;
            let mut off: i64 = 0;
            let mut on: i64 = 0;
            let mut resizing: i64 = 0;
            let mut ok: bool = true;

            match v.state.as_str() {
                "corrupt" => {
                    corrupt = 1;
                }
                "initializing" => {
                    initializing = 1;
                }
                "mixed" => {
                    mixed = 1;
                }
                "off" => {
                    off = 1;
                }
                "on" => {
                    on = 1;
                }
                "resizing" => {
                    resizing = 1;
                }
                _ => {
                    ok = false;
                    error!(
                        "Invalid value {} for volume quota state for volume {} on {}",
                        v.state, vol.name, filer.name
                    );
                }
            };
            if ok {
                exporter::VOLUME_QUOTA_STATE
                    .with_label_values(&[&filer.name, &vol.name, "corrupt"])
                    .set(corrupt);
                exporter::VOLUME_QUOTA_STATE
                    .with_label_values(&[&filer.name, &vol.name, "initializing"])
                    .set(initializing);
                exporter::VOLUME_QUOTA_STATE
                    .with_label_values(&[&filer.name, &vol.name, "mixed"])
                    .set(mixed);
                exporter::VOLUME_QUOTA_STATE
                    .with_label_values(&[&filer.name, &vol.name, "off"])
                    .set(off);
                exporter::VOLUME_QUOTA_STATE
                    .with_label_values(&[&filer.name, &vol.name, "on"])
                    .set(on);
                exporter::VOLUME_QUOTA_STATE
                    .with_label_values(&[&filer.name, &vol.name, "resizing"])
                    .set(resizing);
            }
        }

        if let Some(v) = vol.efficiency {
            if let Some(c) = v.compression {
                let mut inline: i64 = 0;
                let mut background: i64 = 0;
                let mut both: i64 = 0;
                let mut none: i64 = 0;
                let mut mixed: i64 = 0;
                let mut ok: bool = true;

                debug!(
                    "Updating metrics for volume efficiency compression {} {} -> {}",
                    filer.name, vol.name, c
                );
                match c.as_str() {
                    "inline" => {
                        inline = 1;
                    }
                    "background" => {
                        background = 1;
                    }
                    "both" => {
                        both = 1;
                    }
                    "none" => {
                        none = 1;
                    }
                    "mixed" => {
                        mixed = 1;
                    }
                    _ => {
                        ok = false;
                        error!(
                            "Invalid value {} for volume compression for volme {} on {}",
                            c, vol.name, filer.name
                        );
                    }
                };
                if ok {
                    exporter::VOLUME_EFFICIENCY_COMPRESSION
                        .with_label_values(&[&filer.name, &vol.name, "inline"])
                        .set(inline);
                    exporter::VOLUME_EFFICIENCY_COMPRESSION
                        .with_label_values(&[&filer.name, &vol.name, "background"])
                        .set(background);
                    exporter::VOLUME_EFFICIENCY_COMPRESSION
                        .with_label_values(&[&filer.name, &vol.name, "both"])
                        .set(both);
                    exporter::VOLUME_EFFICIENCY_COMPRESSION
                        .with_label_values(&[&filer.name, &vol.name, "none"])
                        .set(none);
                    exporter::VOLUME_EFFICIENCY_COMPRESSION
                        .with_label_values(&[&filer.name, &vol.name, "mixed"])
                        .set(mixed);
                }
            }
            if let Some(c) = v.compaction {
                let mut inline: i64 = 0;
                let mut none: i64 = 0;
                let mut mixed: i64 = 0;
                let mut ok: bool = true;

                debug!(
                    "Updating metrics for volume efficiency compaction {} {} -> {}",
                    filer.name, vol.name, c
                );
                match c.as_str() {
                    "inline" => {
                        inline = 1;
                    }
                    "none" => {
                        none = 1;
                    }
                    "mixed" => {
                        mixed = 1;
                    }
                    _ => {
                        ok = false;
                        error!(
                            "Invalid value {} for volume compaction for volme {} on {}",
                            c, vol.name, filer.name
                        );
                    }
                };
                if ok {
                    exporter::VOLUME_EFFICIENCY_COMPACTION
                        .with_label_values(&[&filer.name, &vol.name, "inline"])
                        .set(inline);
                    exporter::VOLUME_EFFICIENCY_COMPACTION
                        .with_label_values(&[&filer.name, &vol.name, "none"])
                        .set(none);
                    exporter::VOLUME_EFFICIENCY_COMPACTION
                        .with_label_values(&[&filer.name, &vol.name, "mixed"])
                        .set(mixed);
                }
            }
            if let Some(d) = v.dedupe {
                let mut inline: i64 = 0;
                let mut background: i64 = 0;
                let mut both: i64 = 0;
                let mut none: i64 = 0;
                let mut mixed: i64 = 0;
                let mut ok: bool = true;

                debug!(
                    "Updating metrics for volume efficiency dedupe {} {} -> {}",
                    filer.name, vol.name, d
                );
                match d.as_str() {
                    "inline" => {
                        inline = 1;
                    }
                    "background" => {
                        background = 1;
                    }
                    "both" => {
                        both = 1;
                    }
                    "none" => {
                        none = 1;
                    }
                    "mixed" => {
                        mixed = 1;
                    }
                    _ => {
                        ok = false;
                        error!(
                            "Invalid value {} for volume dedupe for volme {} on {}",
                            d, vol.name, filer.name
                        );
                    }
                };
                if ok {
                    exporter::VOLUME_EFFICIENCY_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "inline"])
                        .set(inline);
                    exporter::VOLUME_EFFICIENCY_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "background"])
                        .set(background);
                    exporter::VOLUME_EFFICIENCY_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "both"])
                        .set(both);
                    exporter::VOLUME_EFFICIENCY_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "none"])
                        .set(none);
                    exporter::VOLUME_EFFICIENCY_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "mixed"])
                        .set(mixed);
                }
            }
            if let Some(d) = v.cross_volume_dedupe {
                let mut inline: i64 = 0;
                let mut background: i64 = 0;
                let mut both: i64 = 0;
                let mut none: i64 = 0;
                let mut mixed: i64 = 0;
                let mut ok: bool = true;

                debug!(
                    "Updating metrics for volume efficiency cross_volume_dedupe {} {} -> {}",
                    filer.name, vol.name, d
                );
                match d.as_str() {
                    "inline" => {
                        inline = 1;
                    }
                    "background" => {
                        background = 1;
                    }
                    "both" => {
                        both = 1;
                    }
                    "none" => {
                        none = 1;
                    }
                    "mixed" => {
                        mixed = 1;
                    }
                    _ => {
                        ok = false;
                        error!(
                            "Invalid value {} for volume cross_volume_dedupe for volme {} on {}",
                            d, vol.name, filer.name
                        );
                    }
                };
                if ok {
                    exporter::VOLUME_EFFICIENCY_CROSS_VOLUME_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "inline"])
                        .set(inline);
                    exporter::VOLUME_EFFICIENCY_CROSS_VOLUME_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "background"])
                        .set(background);
                    exporter::VOLUME_EFFICIENCY_CROSS_VOLUME_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "both"])
                        .set(both);
                    exporter::VOLUME_EFFICIENCY_CROSS_VOLUME_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "none"])
                        .set(none);
                    exporter::VOLUME_EFFICIENCY_CROSS_VOLUME_DEDUPE
                        .with_label_values(&[&filer.name, &vol.name, "mixed"])
                        .set(mixed);
                }
            }
        }

        if let Some(v) = vol.metric {
            if v.status == "ok" {
                debug!(
                    "Updating metrics for volume metric duration: {} {} -> {}",
                    filer.name, vol.name, v.duration
                );
                let mut ok: bool = true;

                let duration: i64 = match v.duration.as_str() {
                    "PT15S" => 15,
                    "PT1D" => 86400,
                    "PT2H" => 7200,
                    "PT30M" => 1800,
                    "PT4M" => 240,
                    "PT5M" => 300,
                    _ => {
                        ok = false;
                        error!(
                            "Invalid or unsupported sample duration {} for volume {} on {}",
                            v.duration, vol.name, filer.name
                        );
                        -1
                    }
                };
                if ok {
                    exporter::VOLUME_METRIC_SAMPLE_DURATION
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(duration);
                }

                debug!(
                    "Updating metrics for volume metric throughput read: {} {} -> {}",
                    filer.name, vol.name, v.throughput.read
                );
                exporter::VOLUME_METRIC_THROUGHPUT_READ
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.throughput.read);

                debug!(
                    "Updating metrics for volume metric throughput write: {} {} -> {}",
                    filer.name, vol.name, v.throughput.write
                );
                exporter::VOLUME_METRIC_THROUGHPUT_WRITE
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.throughput.write);

                debug!(
                    "Updating metrics for volume metric throughput other: {} {} -> {}",
                    filer.name, vol.name, v.throughput.other
                );
                exporter::VOLUME_METRIC_THROUGHPUT_OTHER
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.throughput.other);

                debug!(
                    "Updating metrics for volume metric throughput total: {} {} -> {}",
                    filer.name, vol.name, v.throughput.total
                );
                exporter::VOLUME_METRIC_THROUGHPUT_TOTAL
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.throughput.total);

                debug!(
                    "Updating metrics for volume metric latency read: {} {} -> {}",
                    filer.name, vol.name, v.latency.read
                );
                exporter::VOLUME_METRIC_LATENCY_READ
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.latency.read as f64 / 1e+06);

                debug!(
                    "Updating metrics for volume metric latency write: {} {} -> {}",
                    filer.name, vol.name, v.latency.write
                );
                exporter::VOLUME_METRIC_LATENCY_WRITE
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.latency.write as f64 / 1e+06);

                debug!(
                    "Updating metrics for volume metric latency other: {} {} -> {}",
                    filer.name, vol.name, v.latency.other
                );
                exporter::VOLUME_METRIC_LATENCY_OTHER
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.latency.other as f64 / 1e+06);

                debug!(
                    "Updating metrics for volume metric latency total: {} {} -> {}",
                    filer.name, vol.name, v.latency.total
                );
                exporter::VOLUME_METRIC_LATENCY_TOTAL
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.latency.total as f64 / 1e+06);

                debug!(
                    "Updating metrics for volume metric iops read: {} {} -> {}",
                    filer.name, vol.name, v.iops.read
                );
                exporter::VOLUME_METRIC_IOPS_READ
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.iops.read);
                debug!(
                    "Updating metrics for volume metric iops write: {} {} -> {}",
                    filer.name, vol.name, v.iops.write
                );
                exporter::VOLUME_METRIC_IOPS_WRITE
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.iops.write);

                debug!(
                    "Updating metrics for volume metric iops other: {} {} -> {}",
                    filer.name, vol.name, v.iops.other
                );
                exporter::VOLUME_METRIC_IOPS_OTHER
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.iops.other);

                debug!(
                    "Updating metrics for volume metric iops total: {} {} -> {}",
                    filer.name, vol.name, v.iops.total
                );
                exporter::VOLUME_METRIC_IOPS_TOTAL
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(v.iops.total);

                if let Some(vc) = v.cloud {
                    if vc.status == "ok" {
                        debug!(
                            "Updating metrics for volume metric cloud duration: {} {} -> {}",
                            filer.name, vol.name, vc.duration
                        );
                        let mut ok: bool = true;

                        let duration: i64 = match vc.duration.as_str() {
                            "PT15S" => 15,
                            "PT1D" => 86400,
                            "PT2H" => 7200,
                            "PT30M" => 1800,
                            "PT4M" => 240,
                            "PT5M" => 300,
                            _ => {
                                ok = false;
                                error!(
                                    "Invalid or unsupported sample cloud storage duration {} for volume {} on {}",
                                    vc.duration, vol.name, filer.name
                                );
                                -1
                            }
                        };
                        if ok {
                            exporter::VOLUME_METRIC_CLOUD_SAMPLE_DURATION
                                .with_label_values(&[&filer.name, &vol.name])
                                .set(duration);
                        }

                        debug!(
                            "Updating metrics for volume metric cloud latency read: {} {} -> {}",
                            filer.name, vol.name, vc.latency.read
                        );
                        exporter::VOLUME_METRIC_CLOUD_LATENCY_READ
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(vc.latency.read as f64 / 1e+06);

                        debug!(
                            "Updating metrics for volume metric cloud latency write: {} {} -> {}",
                            filer.name, vol.name, vc.latency.write
                        );
                        exporter::VOLUME_METRIC_CLOUD_LATENCY_WRITE
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(vc.latency.write as f64 / 1e+06);

                        debug!(
                            "Updating metrics for volume metric cloud latency other: {} {} -> {}",
                            filer.name, vol.name, vc.latency.other
                        );
                        exporter::VOLUME_METRIC_CLOUD_LATENCY_OTHER
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(vc.latency.other as f64 / 1e+06);

                        debug!(
                            "Updating metrics for volume metric cloud latency total: {} {} -> {}",
                            filer.name, vol.name, vc.latency.total
                        );
                        exporter::VOLUME_METRIC_CLOUD_LATENCY_TOTAL
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(vc.latency.total as f64 / 1e+06);

                        debug!(
                            "Updating metrics for volume metric cloud iops read: {} {} -> {}",
                            filer.name, vol.name, vc.iops.read
                        );
                        exporter::VOLUME_METRIC_CLOUD_IOPS_READ
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(vc.iops.read);

                        debug!(
                            "Updating metrics for volume metric iops write: {} {} -> {}",
                            filer.name, vol.name, vc.iops.write
                        );
                        exporter::VOLUME_METRIC_CLOUD_IOPS_WRITE
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(vc.iops.write);

                        debug!(
                            "Updating metrics for volume metric iops other: {} {} -> {}",
                            filer.name, vol.name, vc.iops.other
                        );
                        exporter::VOLUME_METRIC_CLOUD_IOPS_OTHER
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(vc.iops.other);

                        debug!(
                            "Updating metrics for volume metric iops total: {} {} -> {}",
                            filer.name, vol.name, vc.iops.total
                        );
                        exporter::VOLUME_METRIC_CLOUD_IOPS_TOTAL
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(vc.iops.total);
                    } else {
                        warn!("Skipping metrics from volume {} on {} because metric state was reported as \"{}\" instead of \"ok\"", vol.name, filer.name, v.status);
                    }
                }
                if let Some(vf) = v.flexcache {
                    if vf.status == "ok" {
                        debug!(
                            "Updating metrics for volume metric flexcache duration: {} {} -> {}",
                            filer.name, vol.name, vf.duration
                        );
                        let mut ok: bool = true;

                        let duration: i64 = match vf.duration.as_str() {
                            "PT15S" => 15,
                            "PT1D" => 86400,
                            "PT2H" => 7200,
                            "PT30M" => 1800,
                            "PT4M" => 240,
                            "PT5M" => 300,
                            _ => {
                                ok = false;
                                error!(
                                    "Invalid or unsupported sample flexcache duration {} for volume {} on {}",
                                    vf.duration, vol.name, filer.name
                                );
                                -1
                            }
                        };
                        if ok {
                            exporter::VOLUME_METRIC_FLEXCACHE_SAMPLE_DURATION
                                .with_label_values(&[&filer.name, &vol.name])
                                .set(duration);
                        }

                        debug!("Updating metrics for volume metric flexcache cache_miss_percent {} {} -> {}", filer.name, vol.name, vf.cache_miss_percent);
                        exporter::VOLUME_METRIC_FLEXCACHE_CACHE_MISS_PERCENT
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(vf.cache_miss_percent);
                    } else {
                        warn!("Skipping metrics from volume {} on {} because flexcache metric state was reported as \"{}\" instead of \"ok\"", vol.name, filer.name, vf.status);
                    }
                }
            } else {
                warn!("Skipping metrics from volume {} on {} because metric state was reported as \"{}\" instead of \"ok\"", vol.name, filer.name, v.status);
            }
        }

        if let Some(v) = vol.access_time_enabled {
            debug!(
                "Updating metrics for volume access_time_enabled {} {} -> {}",
                filer.name, vol.name, v
            );
            if v {
                exporter::VOLUME_METRIC_ACCESS_TIME_ENABLED
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(1);
            } else {
                exporter::VOLUME_METRIC_ACCESS_TIME_ENABLED
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(0);
            }
        }

        if let Some(v) = vol.queue_for_encryption {
            debug!(
                "Updating metrics for volume queue_for_encryption {} {} -> {}",
                filer.name, vol.name, v
            );
            if v {
                exporter::VOLUME_METRIC_QUEUE_FOR_ENCRYPTION
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(1);
            } else {
                exporter::VOLUME_METRIC_QUEUE_FOR_ENCRYPTION
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(0);
            }
        }

        if let Some(v) = vol.snaplock {
            debug!(
                "Updating metrics for volume snaplock type {} {} -> {}",
                filer.name, vol.name, v.snaplock_type
            );
            let mut compliance: i64 = 0;
            let mut enterprise: i64 = 0;
            let mut non_snaplock: i64 = 0;
            let mut ok: bool = true;

            match v.snaplock_type.as_str() {
                "compliance" => {
                    compliance = 1;
                }
                "enterprise" => {
                    enterprise = 1;
                }
                "non_snaplock" => {
                    non_snaplock = 1;
                }
                _ => {
                    ok = false;
                    error!(
                        "Invalid snaplock type {} on volume {} of filer {}",
                        v.snaplock_type, vol.name, filer.name
                    );
                }
            };
            if ok {
                exporter::VOLUME_METRIC_SNAPLOCK_TYPE
                    .with_label_values(&[&filer.name, &vol.name, "compliance"])
                    .set(compliance);
                exporter::VOLUME_METRIC_SNAPLOCK_TYPE
                    .with_label_values(&[&filer.name, &vol.name, "enterprise"])
                    .set(enterprise);
                exporter::VOLUME_METRIC_SNAPLOCK_TYPE
                    .with_label_values(&[&filer.name, &vol.name, "non_snaplock"])
                    .set(non_snaplock);
            }

            if let Some(am) = v.append_mode_enabled {
                debug!(
                    "Updating metrics for volume snaplock append_mode_enabled {} {} -> {}",
                    filer.name, vol.name, am
                );
                if am {
                    exporter::VOLUME_METRIC_SNAPLOCK_APPEND_MODE_ENABLED
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(1);
                } else {
                    exporter::VOLUME_METRIC_SNAPLOCK_APPEND_MODE_ENABLED
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(0);
                }
            }

            if let Some(lc) = v.litigation_count {
                debug!(
                    "Updating metrics for volume snaplock litigation_count {} {} -> {}",
                    filer.name, vol.name, lc
                );
                exporter::VOLUME_METRIC_SNAPLOCK_LITIGATION_COUNT
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(lc);
            }

            if let Some(urfc) = v.unspecified_retention_file_count {
                debug!("Updating metrics for volume snaplock unspecified_retention_file_count {} {} -> {}", filer.name, vol.name, urfc);
                exporter::VOLUME_METRIC_SNAPLOCK_UNSPECIFIED_RETENTION_FILE_COUNT
                    .with_label_values(&[&filer.name, &vol.name])
                    .set(urfc);
            }

            if let Some(al) = v.is_audit_log {
                debug!(
                    "Updating metrics for volume snaplock is_audit_log {} {} -> {}",
                    filer.name, vol.name, al
                );
                if al {
                    exporter::VOLUME_METRIC_SNAPLOCK_IS_AUDIT_LOG
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(1);
                } else {
                    exporter::VOLUME_METRIC_SNAPLOCK_IS_AUDIT_LOG
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(0);
                }
            }

            if let Some(pd) = v.privileged_delete {
                debug!(
                    "Updating metrics for volume snaplock privileged_delete {} {} -> {}",
                    filer.name, vol.name, pd
                );
                let mut disabled: i64 = 0;
                let mut enabled: i64 = 0;
                let mut permanently_disabled: i64 = 0;
                let mut ok: bool = true;

                match pd.as_str() {
                    "disabled" => {
                        disabled = 1;
                    }
                    "enabled" => {
                        enabled = 1;
                    }
                    "permanently_disabled" => {
                        permanently_disabled = 1;
                    }
                    _ => {
                        ok = false;
                        error!(
                            "Invalid snaplock privileged_delete value {} on volume {} of filer {}",
                            pd, vol.name, filer.name
                        );
                    }
                };
                if ok {
                    exporter::VOLUME_METRIC_SNAPLOCK_PRIVILEGED_DELETE_TYPE
                        .with_label_values(&[&filer.name, &vol.name, "disabled"])
                        .set(disabled);
                    exporter::VOLUME_METRIC_SNAPLOCK_PRIVILEGED_DELETE_TYPE
                        .with_label_values(&[&filer.name, &vol.name, "enabled"])
                        .set(enabled);
                    exporter::VOLUME_METRIC_SNAPLOCK_PRIVILEGED_DELETE_TYPE
                        .with_label_values(&[&filer.name, &vol.name, "permanently_disabled"])
                        .set(permanently_disabled);
                }
            }

            if let Some(mv) = vol.movement {
                if let Some(v) = mv.percent_complete {
                    debug!(
                        "Updating metrics for volume movement percent_complete {} {} - {}",
                        filer.name, vol.name, v
                    );
                    exporter::VOLUME_METRIC_MOVEMENT_PERCENT_COMPLETE
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(v);
                }

                if let Some(v) = mv.cutover_window {
                    debug!(
                        "Updating metrics for volume movement cutover_window {} {} -> {}",
                        filer.name, vol.name, v
                    );
                    exporter::VOLUME_METRIC_MOVEMENT_CUTOVER_WINDOW
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(v);
                }
                if let Some(v) = mv.tiering_policy {
                    debug!(
                        "Updating metrics for volume movement tiering_policy {} {} -> {}",
                        filer.name, vol.name, v
                    );
                    let mut all: i64 = 0;
                    let mut auto: i64 = 0;
                    let mut backup: i64 = 0;
                    let mut none: i64 = 0;
                    let mut snapshot_only: i64 = 0;
                    let mut ok: bool = true;

                    match v.as_str() {
                        "all" => {
                            all = 1;
                        }
                        "auto" => {
                            auto = 1;
                        }
                        "backup" => {
                            backup = 1;
                        }
                        "none" => {
                            none = 1;
                        }
                        "snapshot_only" => {
                            snapshot_only = 1;
                        }
                        _ => {
                            ok = false;
                            error!("Invalid value {} for movement tiering_policy on volume {} of filer {}", v, vol.name, filer.name);
                        }
                    };
                    if ok {
                        exporter::VOLUME_METRIC_MOVEMENT_TIERING_POLICY
                            .with_label_values(&[&filer.name, &vol.name, "all"])
                            .set(all);
                        exporter::VOLUME_METRIC_MOVEMENT_TIERING_POLICY
                            .with_label_values(&[&filer.name, &vol.name, "auto"])
                            .set(auto);
                        exporter::VOLUME_METRIC_MOVEMENT_TIERING_POLICY
                            .with_label_values(&[&filer.name, &vol.name, "backup"])
                            .set(backup);
                        exporter::VOLUME_METRIC_MOVEMENT_TIERING_POLICY
                            .with_label_values(&[&filer.name, &vol.name, "none"])
                            .set(none);
                        exporter::VOLUME_METRIC_MOVEMENT_TIERING_POLICY
                            .with_label_values(&[&filer.name, &vol.name, "snapshot_only"])
                            .set(snapshot_only);
                    }
                }

                if let Some(v) = mv.state {
                    debug!(
                        "Updating metrics for volume movement state {} {} -> {}",
                        filer.name, vol.name, v
                    );
                    let mut aborted: i64 = 0;
                    let mut cutover: i64 = 0;
                    let mut cutover_wait: i64 = 0;
                    let mut cutover_pending: i64 = 0;
                    let mut failed: i64 = 0;
                    let mut paused: i64 = 0;
                    let mut queued: i64 = 0;
                    let mut replicating: i64 = 0;
                    let mut success: i64 = 0;
                    let mut ok: bool = true;

                    match v.as_str() {
                        "aborted" => {
                            aborted = 1;
                        }
                        "cutover" => {
                            cutover = 1;
                        }
                        "cutover_wait" => {
                            cutover_wait = 1;
                        }
                        "cutover_pending" => {
                            cutover_pending = 1;
                        }
                        "failed" => {
                            failed = 1;
                        }
                        "paused" => {
                            paused = 1;
                        }
                        "queued" => {
                            queued = 1;
                        }
                        "replicating" => {
                            replicating = 1;
                        }
                        "success" => {
                            success = 1;
                        }
                        _ => {
                            ok = false;
                            error!(
                                "Invalid value {} for movement state on volume {} of filer {}",
                                v, vol.name, filer.name
                            );
                        }
                    };
                    if ok {
                        exporter::VOLUME_METRIC_MOVEMENT_STATE
                            .with_label_values(&[&filer.name, &vol.name, "aborted"])
                            .set(aborted);
                        exporter::VOLUME_METRIC_MOVEMENT_STATE
                            .with_label_values(&[&filer.name, &vol.name, "cutover"])
                            .set(cutover);
                        exporter::VOLUME_METRIC_MOVEMENT_STATE
                            .with_label_values(&[&filer.name, &vol.name, "cutover_wait"])
                            .set(cutover_wait);
                        exporter::VOLUME_METRIC_MOVEMENT_STATE
                            .with_label_values(&[&filer.name, &vol.name, "cutover_pending"])
                            .set(cutover_pending);
                        exporter::VOLUME_METRIC_MOVEMENT_STATE
                            .with_label_values(&[&filer.name, &vol.name, "failed"])
                            .set(failed);
                        exporter::VOLUME_METRIC_MOVEMENT_STATE
                            .with_label_values(&[&filer.name, &vol.name, "paused"])
                            .set(paused);
                        exporter::VOLUME_METRIC_MOVEMENT_STATE
                            .with_label_values(&[&filer.name, &vol.name, "queued"])
                            .set(queued);
                        exporter::VOLUME_METRIC_MOVEMENT_STATE
                            .with_label_values(&[&filer.name, &vol.name, "replicating"])
                            .set(replicating);
                        exporter::VOLUME_METRIC_MOVEMENT_STATE
                            .with_label_values(&[&filer.name, &vol.name, "success"])
                            .set(success);
                    }
                }
            }

            if let Some(v) = vol.style {
                debug!(
                    "Updating metrics for volume style {} {} -> {}",
                    filer.name, vol.name, v
                );
                let mut ok: bool = true;
                let mut flexvol: i64 = 0;
                let mut flexgroup: i64 = 0;

                match v.as_str() {
                    "flexvol" => {
                        flexvol = 1;
                    }
                    "flexgroup" => {
                        flexgroup = 1;
                    }
                    _ => {
                        ok = false;
                        error!(
                            "Invalid value {} for volume style for volume {} on filer {}",
                            v, vol.name, filer.name
                        );
                    }
                };

                if ok {
                    exporter::VOLUME_METRIC_STYLE
                        .with_label_values(&[&filer.name, &vol.name, "flexvol"])
                        .set(flexvol);
                    exporter::VOLUME_METRIC_STYLE
                        .with_label_values(&[&filer.name, &vol.name, "flexgroup"])
                        .set(flexgroup);
                }
            }

            if let Some(enc) = vol.encryption {
                if let Some(tpe) = enc.enc_type {
                    let mut none: i64 = 0;
                    let mut volume: i64 = 0;
                    let mut aggregate: i64 = 0;
                    let mut ok: bool = true;

                    debug!(
                        "Updating metrics for volume enryption type {} {} -> {}",
                        filer.name, vol.name, tpe
                    );
                    match tpe.as_str() {
                        "none" => {
                            none = 1;
                        }
                        "volume" => {
                            volume = 1;
                        }
                        "aggregate" => {
                            aggregate = 1;
                        }
                        _ => {
                            error!(
                                "Invalid value {} for encryption type for volume {} on filer {}",
                                tpe, vol.name, filer.name
                            );
                            ok = false;
                        }
                    };
                    if ok {
                        exporter::VOLUME_METRIC_ENCRYPTION_TYPE
                            .with_label_values(&[&filer.name, &vol.name, "none"])
                            .set(none);
                        exporter::VOLUME_METRIC_ENCRYPTION_TYPE
                            .with_label_values(&[&filer.name, &vol.name, "volume"])
                            .set(volume);
                        exporter::VOLUME_METRIC_ENCRYPTION_TYPE
                            .with_label_values(&[&filer.name, &vol.name, "aggregate"])
                            .set(aggregate);
                    }
                }

                if let Some(state) = enc.state {
                    let mut encrypted: i64 = 0;
                    let mut encrypting: i64 = 0;
                    let mut partial: i64 = 0;
                    let mut rekeying: i64 = 0;
                    let mut unencrypted: i64 = 0;
                    let mut ok: bool = true;

                    debug!(
                        "Updating metrics for volume encryption state {} {} -> {}",
                        filer.name, vol.name, state
                    );
                    match state.as_str() {
                        "encrypted" => {
                            encrypted = 1;
                        }
                        "encrypting" => {
                            encrypting = 1;
                        }
                        "partial" => {
                            partial = 1;
                        }
                        "rekeying" => {
                            rekeying = 1;
                        }
                        "unencrypted" => {
                            unencrypted = 1;
                        }
                        _ => {
                            error!(
                                "Invalid value {} for encryption state for volume {} on filer {}",
                                state, vol.name, filer.name
                            );
                            ok = false;
                        }
                    };
                    if ok {
                        exporter::VOLUME_METRIC_ENCRYPTION_STATE
                            .with_label_values(&[&filer.name, &vol.name, "encrypted"])
                            .set(encrypted);
                        exporter::VOLUME_METRIC_ENCRYPTION_STATE
                            .with_label_values(&[&filer.name, &vol.name, "encrypting"])
                            .set(encrypting);
                        exporter::VOLUME_METRIC_ENCRYPTION_STATE
                            .with_label_values(&[&filer.name, &vol.name, "partial"])
                            .set(partial);
                        exporter::VOLUME_METRIC_ENCRYPTION_STATE
                            .with_label_values(&[&filer.name, &vol.name, "rekeying"])
                            .set(rekeying);
                        exporter::VOLUME_METRIC_ENCRYPTION_STATE
                            .with_label_values(&[&filer.name, &vol.name, "unencrypted"])
                            .set(unencrypted);
                    }
                }

                debug!(
                    "Updating metrics for volume encryption enabled {} {} -> {}",
                    filer.name, vol.name, enc.enabled
                );
                if enc.enabled {
                    exporter::VOLUME_METRIC_ENCRYPTION_ENABLED
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(1);
                } else {
                    exporter::VOLUME_METRIC_ENCRYPTION_ENABLED
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(0);
                }
            }

            if let Some(tier) = vol.tiering {
                debug!(
                    "Updating metrics for volume tiering policy {} {} -> {}",
                    filer.name, vol.name, tier.policy
                );
                let mut all: i64 = 0;
                let mut auto: i64 = 0;
                let mut backup: i64 = 0;
                let mut none: i64 = 0;
                let mut snapshot_only: i64 = 0;
                let mut ok: bool = true;
                match tier.policy.as_str() {
                    "all" => {
                        all = 1;
                    }
                    "auto" => {
                        auto = 1;
                    }
                    "backup" => {
                        backup = 1;
                    }
                    "none" => {
                        none = 1;
                    }
                    "snapshot_only" => {
                        snapshot_only = 1;
                    }
                    _ => {
                        error!(
                            "Invalid value {} for volume tiering policy for volume {} on filer {}",
                            filer.name, vol.name, tier.policy
                        );
                        ok = false;
                    }
                };
                if ok {
                    exporter::VOLUME_METRIC_TIERING_POLICY
                        .with_label_values(&[&filer.name, &vol.name, "all"])
                        .set(all);
                    exporter::VOLUME_METRIC_TIERING_POLICY
                        .with_label_values(&[&filer.name, &vol.name, "auto"])
                        .set(auto);
                    exporter::VOLUME_METRIC_TIERING_POLICY
                        .with_label_values(&[&filer.name, &vol.name, "backup"])
                        .set(backup);
                    exporter::VOLUME_METRIC_TIERING_POLICY
                        .with_label_values(&[&filer.name, &vol.name, "none"])
                        .set(none);
                    exporter::VOLUME_METRIC_TIERING_POLICY
                        .with_label_values(&[&filer.name, &vol.name, "snapshot_only"])
                        .set(snapshot_only);
                }

                if let Some(spt) = tier.supported {
                    debug!(
                        "Updating volume metrics tiering supported {} {} -> {}",
                        filer.name, vol.name, spt
                    );
                    if spt {
                        exporter::VOLUME_METRIC_TIERING_SUPPORTED
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(1);
                    } else {
                        exporter::VOLUME_METRIC_TIERING_SUPPORTED
                            .with_label_values(&[&filer.name, &vol.name])
                            .set(0);
                    }
                }

                if let Some(min) = tier.min_cooling_days {
                    debug!(
                        "Updating metrics for volume tiering min_cooling_days {} {} -> {}",
                        filer.name, vol.name, min
                    );
                    exporter::VOLUME_METRIC_TIERING_MIN_COOLING_DAYS
                        .with_label_values(&[&filer.name, &vol.name])
                        .set(min);
                }
            }
        }
    }
    Ok(())
}
