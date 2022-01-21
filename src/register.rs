use crate::config;
use crate::constants;

/*
pub fn targets(register: &mut config::ScrapeTargets, target: &config::ScrapeTargets) {
    if let Some(val) = target.aggregates {
        if val {
            register.aggregates = match register.aggregates {
                Some(v) => Some(v),
                None => Some(true),
            };
        }
    }
    if let Some(val) = target.volumes {
        if val {
            register.volumes = match register.volumes {
                Some(v) => Some(v),
                None => Some(true),
            };
        }
    }
}
*/

pub fn build_target_masks(scrape: &config::ScrapeTargets) -> u64 {
    let mut result: u64 = 0;

    if let Some(val) = scrape.aggregates {
        if val {
            result |= constants::TARGET_AGGREGATES;
        }
    }
    if let Some(val) = scrape.volumes {
        if val {
            result |= constants::TARGET_VOLUMES;
        }
    }
    result
}
