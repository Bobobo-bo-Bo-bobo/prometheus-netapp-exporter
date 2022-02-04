use crate::config;
use crate::constants;
use crate::exporter;
use crate::http;

use log::{debug, error};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Clone, Debug)]
pub struct QuotaList {
    pub records: Vec<Quota>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Quota {
    pub files: Option<QuotaUsage>,
    pub space: Option<QuotaUsage>,
    pub users: Option<Vec<QuotaUser>>,
    pub group: Option<QuotaGroup>,
    pub qtree: Option<QuotaQTree>,
    #[serde(rename = "type")]
    pub quota_type: String,
    pub volume: QuotaVolume,
    pub svm: QuotaSVM,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuotaUser {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuotaGroup {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuotaQTree {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuotaUsage {
    pub hard_limit: Option<i64>,
    pub soft_limit: Option<i64>,
    pub used: Option<QuotaUsagePercent>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuotaUsagePercent {
    pub soft_limit_percent: Option<i64>,
    pub hard_limit_percent: Option<i64>,
    pub total: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuotaVolume {
    pub name: String,
    pub uuid: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuotaSVM {
    pub name: String,
    pub uuid: String,
}

pub fn update_quotas(
    filer: &config::NetAppConfiguration,
    client: &mut reqwest::blocking::Client,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://{}{}?fields=**",
        filer.address,
        constants::API_QUOTAS
    );
    let raw_quotas = match http::get(client, &url, &filer.user, &filer.password) {
        Ok(v) => v,
        Err(e) => {
            bail!("Request for quotas on {} failed - {}", filer.name, e);
        }
    };
    let quotas: QuotaList = match serde_json::from_str(&raw_quotas) {
        Ok(v) => v,
        Err(e) => bail!(
            "Can't decode response for quota information from {} as JSON - {}",
            filer.name,
            e
        ),
    };

    for quota in quotas.records {
        match quota.quota_type.as_str() {
            "user" => {
                update_user_quotas(&filer.name, &quota);
            }
            "group" => {
                update_group_quotas(&filer.name, &quota);
            }
            "tree" => {
                update_tree_quotas(&filer.name, &quota);
            }
            _ => {
                error!(
                    "Invalid quota type {} on volume {} of filer {}",
                    quota.quota_type, quota.volume.name, filer.name
                );
                continue;
            }
        };
    }
    Ok(())
}

fn update_user_quotas(filer: &str, quota: &Quota) {
    let qtree_name = match &quota.qtree {
        Some(v) => &v.name,
        None => panic!("Quota has no qtree structure:\n{:?}\n", quota),
    };

    let users = match &quota.users {
        Some(v) => v,
        None => panic!("User quota has no user structure\n{:?}\n", quota),
    };

    // For a default tree quota policy rule, this parameter is specified as “” or "*"
    if qtree_name == "*" || qtree_name.is_empty() {
        debug!(
            "Skipping qtree \"{}\" because it is the default tree quota policy rule",
            qtree_name
        );
        return;
    }
    for user in users {
        let user_name = &user.name;
        if let Some(space) = &quota.space {
            if let Some(used) = &space.used {
                debug!(
                    "Updating metrics for tree quota space used total {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, user_name, used.total
                );
                exporter::QUOTA_USER_METRIC_SPACE_USED
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                    .set(used.total);

                if let Some(uhpct) = used.hard_limit_percent {
                    debug!(
                        "Updating metrics for tree quota space used hard_limit_percent {} {} {} {} -> {}",
                        filer, quota.volume.name, qtree_name, user_name, uhpct
                    );
                    exporter::QUOTA_USER_METRIC_SPACE_HARD_LIMIT_PERCENT
                        .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                        .set(uhpct);
                }

                if let Some(uspct) = used.soft_limit_percent {
                    debug!(
                        "Updating metrics for tree quota space used soft_limit_percent {} {} {} {} -> {}",
                        filer, quota.volume.name, qtree_name, user_name, uspct
                    );
                    exporter::QUOTA_USER_METRIC_SPACE_SOFT_LIMIT_PERCENT
                        .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                        .set(uspct);
                }
            }

            if let Some(shl) = space.hard_limit {
                debug!(
                    "Updating metrics for tree quota space hard_limit {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, user_name, shl
                );
                exporter::QUOTA_USER_METRIC_SPACE_HARD_LIMIT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                    .set(shl);
            }

            if let Some(ssl) = space.soft_limit {
                debug!(
                    "Updating metrics for tree quota space soft_limit {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, user_name, ssl
                );
                exporter::QUOTA_USER_METRIC_SPACE_HARD_LIMIT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                    .set(ssl);
            }
        }

        if let Some(files) = &quota.files {
            if let Some(used) = &files.used {
                debug!(
                    "Updating metrics for tree quota files used total {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, user_name, used.total
                );
                exporter::QUOTA_USER_METRIC_FILES_USED
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                    .set(used.total);

                if let Some(uhpct) = used.hard_limit_percent {
                    debug!(
                        "Updating metrics for tree quota files used hard_limit_percent {} {} {} {} -> {}",
                        filer, quota.volume.name, qtree_name, user_name, uhpct
                    );
                    exporter::QUOTA_USER_METRIC_FILES_HARD_LIMIT_PERCENT
                        .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                        .set(uhpct);
                }

                if let Some(uspct) = used.soft_limit_percent {
                    debug!(
                        "Updating metrics for tree quota files used soft_limit_percent {} {} {} {} -> {}",
                        filer, quota.volume.name, qtree_name, user_name, uspct
                    );
                    exporter::QUOTA_USER_METRIC_FILES_SOFT_LIMIT_PERCENT
                        .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                        .set(uspct);
                }
            }

            if let Some(fhl) = files.hard_limit {
                debug!(
                    "Updating metrics for tree quota files hard_limit {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, user_name, fhl
                );
                exporter::QUOTA_USER_METRIC_FILES_HARD_LIMIT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                    .set(fhl);
            }

            if let Some(fsl) = files.soft_limit {
                debug!(
                    "Updating metrics for tree quota files soft_limit {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, user_name, fsl
                );
                exporter::QUOTA_USER_METRIC_FILES_HARD_LIMIT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, user_name])
                    .set(fsl);
            }
        }
    }
}

fn update_group_quotas(filer: &str, quota: &Quota) {
    let qtree_name = match &quota.qtree {
        Some(v) => &v.name,
        None => panic!("Quota has no qtree structure:\n{:?}\n", quota),
    };

    let group_name = match &quota.group {
        Some(v) => &v.name,
        None => panic!("Group quota has no group structure\n{:?}\n", quota),
    };

    // For a default tree quota policy rule, this parameter is specified as “” or "*"
    if qtree_name == "*" || qtree_name.is_empty() {
        debug!(
            "Skipping qtree \"{}\" because it is the default tree quota policy rule",
            qtree_name
        );
        return;
    }

    if let Some(space) = &quota.space {
        if let Some(used) = &space.used {
            debug!(
                "Updating metrics for tree quota space used total {} {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, group_name, used.total
            );
            exporter::QUOTA_GROUP_METRIC_SPACE_USED
                .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                .set(used.total);

            if let Some(uhpct) = used.hard_limit_percent {
                debug!(
                    "Updating metrics for tree quota space used hard_limit_percent {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, group_name, uhpct
                );
                exporter::QUOTA_GROUP_METRIC_SPACE_HARD_LIMIT_PERCENT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                    .set(uhpct);
            }

            if let Some(uspct) = used.soft_limit_percent {
                debug!(
                    "Updating metrics for tree quota space used soft_limit_percent {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, group_name, uspct
                );
                exporter::QUOTA_GROUP_METRIC_SPACE_SOFT_LIMIT_PERCENT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                    .set(uspct);
            }
        }

        if let Some(shl) = space.hard_limit {
            debug!(
                "Updating metrics for tree quota space hard_limit {} {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, group_name, shl
            );
            exporter::QUOTA_GROUP_METRIC_SPACE_HARD_LIMIT
                .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                .set(shl);
        }

        if let Some(ssl) = space.soft_limit {
            debug!(
                "Updating metrics for tree quota space soft_limit {} {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, group_name, ssl
            );
            exporter::QUOTA_GROUP_METRIC_SPACE_HARD_LIMIT
                .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                .set(ssl);
        }
    }

    if let Some(files) = &quota.files {
        if let Some(used) = &files.used {
            debug!(
                "Updating metrics for tree quota files used total {} {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, group_name, used.total
            );
            exporter::QUOTA_GROUP_METRIC_FILES_USED
                .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                .set(used.total);

            if let Some(uhpct) = used.hard_limit_percent {
                debug!(
                    "Updating metrics for tree quota files used hard_limit_percent {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, group_name, uhpct
                );
                exporter::QUOTA_GROUP_METRIC_FILES_HARD_LIMIT_PERCENT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                    .set(uhpct);
            }

            if let Some(uspct) = used.soft_limit_percent {
                debug!(
                    "Updating metrics for tree quota files used soft_limit_percent {} {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, group_name, uspct
                );
                exporter::QUOTA_GROUP_METRIC_FILES_SOFT_LIMIT_PERCENT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                    .set(uspct);
            }
        }

        if let Some(fhl) = files.hard_limit {
            debug!(
                "Updating metrics for tree quota files hard_limit {} {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, group_name, fhl
            );
            exporter::QUOTA_GROUP_METRIC_FILES_HARD_LIMIT
                .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                .set(fhl);
        }

        if let Some(fsl) = files.soft_limit {
            debug!(
                "Updating metrics for tree quota files soft_limit {} {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, group_name, fsl
            );
            exporter::QUOTA_GROUP_METRIC_FILES_HARD_LIMIT
                .with_label_values(&[filer, &quota.volume.name, qtree_name, group_name])
                .set(fsl);
        }
    }
}

fn update_tree_quotas(filer: &str, quota: &Quota) {
    let qtree_name = match &quota.qtree {
        Some(v) => &v.name,
        None => panic!("Quota has no qtree structure:\n{:?}\n", quota),
    };

    // For a default tree quota policy rule, this parameter is specified as “” or "*"
    if qtree_name == "*" || qtree_name.is_empty() {
        debug!(
            "Skipping qtree \"{}\" because it is the default tree quota policy rule",
            qtree_name
        );
        return;
    }

    if let Some(space) = &quota.space {
        if let Some(used) = &space.used {
            debug!(
                "Updating metrics for tree quota space used total {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, used.total
            );
            exporter::QUOTA_TREE_METRIC_SPACE_USED
                .with_label_values(&[filer, &quota.volume.name, qtree_name])
                .set(used.total);

            if let Some(uhpct) = used.hard_limit_percent {
                debug!(
                    "Updating metrics for tree quota space used hard_limit_percent {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, uhpct
                );
                exporter::QUOTA_TREE_METRIC_SPACE_HARD_LIMIT_PERCENT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name])
                    .set(uhpct);
            }

            if let Some(uspct) = used.soft_limit_percent {
                debug!(
                    "Updating metrics for tree quota space used soft_limit_percent {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, uspct
                );
                exporter::QUOTA_TREE_METRIC_SPACE_SOFT_LIMIT_PERCENT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name])
                    .set(uspct);
            }
        }

        if let Some(shl) = space.hard_limit {
            debug!(
                "Updating metrics for tree quota space hard_limit {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, shl
            );
            exporter::QUOTA_TREE_METRIC_SPACE_HARD_LIMIT
                .with_label_values(&[filer, &quota.volume.name, qtree_name])
                .set(shl);
        }

        if let Some(ssl) = space.soft_limit {
            debug!(
                "Updating metrics for tree quota space soft_limit {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, ssl
            );
            exporter::QUOTA_TREE_METRIC_SPACE_HARD_LIMIT
                .with_label_values(&[filer, &quota.volume.name, qtree_name])
                .set(ssl);
        }
    }

    if let Some(files) = &quota.files {
        if let Some(used) = &files.used {
            debug!(
                "Updating metrics for tree quota files used total {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, used.total
            );
            exporter::QUOTA_TREE_METRIC_FILES_USED
                .with_label_values(&[filer, &quota.volume.name, qtree_name])
                .set(used.total);

            if let Some(uhpct) = used.hard_limit_percent {
                debug!(
                    "Updating metrics for tree quota files used hard_limit_percent {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, uhpct
                );
                exporter::QUOTA_TREE_METRIC_FILES_HARD_LIMIT_PERCENT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name])
                    .set(uhpct);
            }

            if let Some(uspct) = used.soft_limit_percent {
                debug!(
                    "Updating metrics for tree quota files used soft_limit_percent {} {} {} -> {}",
                    filer, quota.volume.name, qtree_name, uspct
                );
                exporter::QUOTA_TREE_METRIC_FILES_SOFT_LIMIT_PERCENT
                    .with_label_values(&[filer, &quota.volume.name, qtree_name])
                    .set(uspct);
            }
        }

        if let Some(fhl) = files.hard_limit {
            debug!(
                "Updating metrics for tree quota files hard_limit {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, fhl
            );
            exporter::QUOTA_TREE_METRIC_FILES_HARD_LIMIT
                .with_label_values(&[filer, &quota.volume.name, qtree_name])
                .set(fhl);
        }

        if let Some(fsl) = files.soft_limit {
            debug!(
                "Updating metrics for tree quota files soft_limit {} {} {} -> {}",
                filer, quota.volume.name, qtree_name, fsl
            );
            exporter::QUOTA_TREE_METRIC_FILES_HARD_LIMIT
                .with_label_values(&[filer, &quota.volume.name, qtree_name])
                .set(fsl);
        }
    }
}
