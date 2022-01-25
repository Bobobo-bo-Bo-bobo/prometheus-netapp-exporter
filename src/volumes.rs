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
    pub files: Option<VolumeFiles>,
    pub state: Option<String>,
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
                    error!(
                        "Invalid state {} for volume {} on {}",
                        v, vol.name, filer.name
                    );
                    continue;
                }
            };

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
                "Updating metric for volume autosize minimum {} {} -> {}",
                filer.name, vol.name, v.minimum
            );
            exporter::VOLUME_AUTOSIZE_MIN
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.minimum);

            debug!(
                "Updating metric for volume autosize maximum {} {} -> {}",
                filer.name, vol.name, v.maximum
            );
            exporter::VOLUME_AUTOSIZE_MAX
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.maximum);

            debug!(
                "Updating metric for volume autosize mode {} {} -> {}",
                filer.name, vol.name, v.mode
            );
            let mut grow: i64 = 0;
            let mut grow_shrink: i64 = 0;
            let mut off: i64 = 0;
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
                    error!(
                        "Invalid autosize mode {} for volume {} on {}",
                        v.mode, vol.name, filer.name
                    );
                    continue;
                }
            };
            exporter::VOLUME_AUTOSIZE_MODE
                .with_label_values(&[&filer.name, &vol.name, "grow"])
                .set(grow);
            exporter::VOLUME_AUTOSIZE_MODE
                .with_label_values(&[&filer.name, &vol.name, "grow_shrink"])
                .set(grow_shrink);
            exporter::VOLUME_AUTOSIZE_MODE
                .with_label_values(&[&filer.name, &vol.name, "off"])
                .set(off);

            debug!(
                "Updating metric for volume autosize shrink_threshold {} {} -> {}",
                filer.name, vol.name, v.shrink_threshold
            );
            exporter::VOLUME_AUTOSIZE_SHRINK_THRESHOLD
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.shrink_threshold);

            debug!(
                "Updating metric for volume autosize grow_threshold {} {} -> {}",
                filer.name, vol.name, v.grow_threshold
            );
            exporter::VOLUME_AUTOSIZE_GROW_THRESHOLD
                .with_label_values(&[&filer.name, &vol.name])
                .set(v.grow_threshold);
        }
    }
    Ok(())
}