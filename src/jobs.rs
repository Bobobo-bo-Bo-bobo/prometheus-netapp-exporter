use crate::config;
use crate::constants;
use crate::exporter;
use crate::http;

use log::{debug, error};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Clone, Debug)]
pub struct JobList {
    pub records: Vec<Job>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Job {
    pub uuid: String,
    pub state: String,
}

pub fn update_jobs(
    filer: &config::NetAppConfiguration,
    client: &mut reqwest::blocking::Client,
) -> Result<(), Box<dyn Error>> {
    let url = format!("https://{}{}?fields=**", filer.address, constants::API_JOBS);
    let raw_jobs = match http::get(client, &url, &filer.user, &filer.password) {
        Ok(v) => v,
        Err(e) => {
            bail!("Request for cluster jobs on {} failed - {}", filer.name, e);
        }
    };

    let job_list: JobList = match serde_json::from_str(&raw_jobs) {
        Ok(v) => v,
        Err(e) => bail!(
            "Can't decode response for cluster job information from {} as JSON - {}",
            filer.name,
            e
        ),
    };

    let mut undef_count: i64 = 0;
    let mut queued: i64 = 0;
    let mut running: i64 = 0;
    let mut paused: i64 = 0;
    let mut success: i64 = 0;
    let mut failure: i64 = 0;

    for job in job_list.records {
        match job.state.as_str() {
            "queued" => {
                queued += 1;
            }
            "running" => {
                running += 1;
            }
            "paused" => {
                paused += 1;
            }
            "success" => {
                success += 1;
            }
            "failure" => {
                failure += 1;
            }
            _ => {
                error!(
                    "Invalid job state {} for job {} on filer {}",
                    job.state, job.uuid, filer.name
                );
                undef_count += 1;
            }
        };
    }

    if undef_count == 0 {
        debug!(
            "Updating metrics cluster job {} queued -> {}",
            filer.name, queued
        );
        exporter::CLUSTER_JOB_STATE
            .with_label_values(&[&filer.name, "queued"])
            .set(queued);

        debug!(
            "Updating metrics cluster job {} running -> {}",
            filer.name, running
        );
        exporter::CLUSTER_JOB_STATE
            .with_label_values(&[&filer.name, "running"])
            .set(running);

        debug!(
            "Updating metrics cluster job {} paused -> {}",
            filer.name, paused
        );
        exporter::CLUSTER_JOB_STATE
            .with_label_values(&[&filer.name, "paused"])
            .set(paused);

        debug!(
            "Updating metrics cluster job {} success -> {}",
            filer.name, success
        );
        exporter::CLUSTER_JOB_STATE
            .with_label_values(&[&filer.name, "success"])
            .set(success);

        debug!(
            "Updating metrics cluster job {} failure -> {}",
            filer.name, failure
        );
        exporter::CLUSTER_JOB_STATE
            .with_label_values(&[&filer.name, "failure"])
            .set(failure);
    }
    Ok(())
}
