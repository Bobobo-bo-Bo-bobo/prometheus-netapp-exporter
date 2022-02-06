use crate::config;
use crate::constants;

pub fn build_target_masks(scrape: &config::ScrapeTargets) -> u64 {
    let mut result: u64 = 0;

    if let Some(val) = scrape.aggregates {
        if val {
            result |= constants::TARGET_AGGREGATES;
        }
    }
    if let Some(val) = scrape.quotas {
        if val {
            result |= constants::TARGET_QUOTAS;
        }
    }
    if let Some(val) = scrape.volumes {
        if val {
            result |= constants::TARGET_VOLUMES;
        }
    }
    if let Some(val) = scrape.chassis {
        if val {
            result |= constants::TARGET_CHASSIS;
        }
    }
    if let Some(val) = scrape.jobs {
        if val {
            result |= constants::TARGET_JOBS;
        }
    }
    if let Some(val) = scrape.ethernet {
        if val {
            result |= constants::TARGET_ETHERNET;
        }
    }

    result
}
