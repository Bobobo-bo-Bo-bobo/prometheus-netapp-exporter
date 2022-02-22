use crate::config;
use crate::constants;
use crate::exporter;
use crate::http;

use log::debug;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Clone, Debug)]
pub struct NfsList {
    pub records: Vec<Nfs>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Nfs {
    pub protocol: String,
    pub volume: Option<Volume>,
    pub local_request_count: u64,
    pub remote_request_count: u64,
    pub client_ip: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Volume {
    pub name: String,
}

pub fn update_nfs(
    filer: &config::NetAppConfiguration,
    client: &mut reqwest::blocking::Client,
    client_ip: bool,
) -> Result<(), Box<dyn Error>> {
    let url = format!("https://{}{}?fields=**", filer.address, constants::API_NFS);
    let raw_nfs = match http::get(client, &url, &filer.user, &filer.password) {
        Ok(v) => v,
        Err(e) => {
            bail!(
                "Request for NFS protocol information on {} failed - {}",
                filer.name,
                e
            );
        }
    };

    let nfs_list: NfsList = match serde_json::from_str(&raw_nfs) {
        Ok(v) => v,
        Err(e) => bail!(
            "Can't decode response for CIFS protocol information from {} as JSON - {}",
            filer.name,
            e
        ),
    };

    let mut protocols = HashMap::<String, i64>::new();
    for p in constants::NFS_PROTOCOL_LIST {
        protocols.insert(p.to_string(), 0);
    }

    let mut volumes = HashMap::<String, i64>::new();
    let mut local_request_counts: u64 = 0;
    let mut remote_request_counts: u64 = 0;
    let mut client_ips = HashMap::<String, i64>::new();

    for nfs in nfs_list.records {
        *protocols.entry(nfs.protocol).or_insert(0) += 1;
        // XXX: depending on state of a client connection and protocol, volume entry can be missing
        if let Some(v) = nfs.volume {
            *volumes.entry(v.name).or_insert(0) += 1;
        }
        local_request_counts += nfs.local_request_count;
        remote_request_counts += nfs.remote_request_count;

        if client_ip {
            *client_ips.entry(nfs.client_ip).or_insert(0) += 1;
        }
    }

    for (prot, prot_cnt) in protocols {
        debug!(
            "Updating metrics for nfs protocol -> {} {} {}",
            filer.name, prot, prot_cnt
        );
        exporter::NFS_PROTOCOL
            .with_label_values(&[&filer.name, &prot])
            .set(prot_cnt);
    }

    for (vol, vol_cnt) in volumes {
        debug!(
            "Updating metrics for nfs volume -> {} {} {}",
            filer.name, vol, vol_cnt
        );
        exporter::NFS_VOLUME
            .with_label_values(&[&filer.name, &vol])
            .set(vol_cnt);
    }

    debug!(
        "Updating metrics for nfs local_request_count -> {} {}",
        filer.name, local_request_counts
    );
    let old_local_count = exporter::NFS_LOCAL_COUNT
        .with_label_values(&[&filer.name])
        .get();
    if local_request_counts < old_local_count {
        // Counter wrap
        // XXX: Because we don't know the type/maximal value of the counter used by the filer, we can't calculate the new value. Just reset it.
        exporter::NFS_LOCAL_COUNT
            .with_label_values(&[&filer.name])
            .reset();
    } else {
        exporter::NFS_LOCAL_COUNT
            .with_label_values(&[&filer.name])
            .inc_by(local_request_counts - old_local_count);
    }

    debug!(
        "Updating metrics for nfs remote_request_count -> {} {}",
        filer.name, remote_request_counts
    );
    let old_remote_count = exporter::NFS_REMOTE_COUNT
        .with_label_values(&[&filer.name])
        .get();
    if remote_request_counts < old_remote_count {
        // Counter wrap
        // XXX: Because we don't know the type/maximal value of the counter used by the filer, we can't calculate the new value. Just reset it.
        exporter::NFS_REMOTE_COUNT
            .with_label_values(&[&filer.name])
            .reset();
    } else {
        exporter::NFS_REMOTE_COUNT
            .with_label_values(&[&filer.name])
            .inc_by(remote_request_counts - old_remote_count);
    }

    if client_ip {
        for (ip, ip_cnt) in client_ips {
            debug!(
                "Updating metrics for nfs client_ip -> {} {} {}",
                filer.name, ip, ip_cnt
            );
            exporter::NFS_CLIENT
                .with_label_values(&[&filer.name, &ip])
                .set(ip_cnt);
        }
    }

    Ok(())
}
