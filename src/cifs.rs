use crate::config;
use crate::constants;
use crate::exporter;
use crate::http;

use log::{debug, error, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Clone, Debug)]
pub struct CifsList {
    pub records: Vec<Cifs>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Cifs {
    pub protocol: String,
    pub smb_encryption: String,
    pub continuous_availability: String,
    pub open_shares: i64,
    pub authentication: String,
    pub volumes: Vec<Volume>,
    pub smb_signing: bool,
    pub connection_count: i64,
    pub user: String,
    pub mapped_unix_user: String,
    pub open_files: i64,
    pub client_ip: String,
    pub open_other: i64,
    pub large_mtu: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Volume {
    pub name: String,
}

pub fn update_cifs(
    filer: &config::NetAppConfiguration,
    client: &mut reqwest::blocking::Client,
    mapped_user: bool,
    user: bool,
) -> Result<(), Box<dyn Error>> {
    let url = format!("https://{}{}?fields=**", filer.address, constants::API_CIFS);
    let raw_cifs = match http::get(client, &url, &filer.user, &filer.password) {
        Ok(v) => v,
        Err(e) => {
            bail!(
                "Request for CIFS protocol information on {} failed - {}",
                filer.name,
                e
            );
        }
    };

    let cifs_list: CifsList = match serde_json::from_str(&raw_cifs) {
        Ok(v) => v,
        Err(e) => bail!(
            "Can't decode response for CIFS protocol information from {} as JSON - {}",
            filer.name,
            e
        ),
    };

    let mut clients = HashMap::<String, i64>::new();
    let mut authentications = HashMap::<String, i64>::new();
    let mut protocols = HashMap::<String, i64>::new();
    let mut volumes = HashMap::<String, i64>::new();
    let mut users = HashMap::<String, i64>::new();
    let mut mapped_users = HashMap::<String, i64>::new();
    let mut smb_encryptions = HashMap::<String, i64>::new();
    let mut continuous_availabilities = HashMap::<String, i64>::new();
    let mut open_shares: i64 = 0;
    let mut open_files: i64 = 0;
    let mut open_others: i64 = 0;
    let mut smb_signing_on: i64 = 0;
    let mut smb_signing_off: i64 = 0;
    let mut connections: i64 = 0;
    let mut large_mtu_on: i64 = 0;
    let mut large_mtu_off: i64 = 0;

    for cifs in cifs_list.records {
        *protocols.entry(cifs.protocol).or_insert(0) += 1;
        *smb_encryptions.entry(cifs.smb_encryption).or_insert(0) += 1;
        *continuous_availabilities
            .entry(cifs.continuous_availability)
            .or_insert(0) += 1;
        open_shares += cifs.open_shares;
        open_files += cifs.open_files;
        open_others += cifs.open_other;
        *authentications.entry(cifs.authentication).or_insert(0) += 1;
        for vol in cifs.volumes {
            *volumes.entry(vol.name).or_insert(0) += 1;
        }
        if cifs.smb_signing {
            smb_signing_on += 1;
        } else {
            smb_signing_off += 1;
        }
        connections += cifs.connection_count;
        if user {
            *users.entry(cifs.user).or_insert(0) += 1;
        }
        if mapped_user {
            *mapped_users.entry(cifs.mapped_unix_user).or_insert(0) += 1;
        }
        *clients.entry(cifs.client_ip).or_insert(0) += 1;
        if cifs.large_mtu {
            large_mtu_on += 1;
        } else {
            large_mtu_off += 1;
        }
    }

    for cifs_proto in constants::CIFS_PROTOCOLS {
        let cifs_proto_count = protocols.get(cifs_proto).unwrap_or(&0);
        debug!(
            "Updating metrics for cifs protocol -> {} {} {}",
            filer.name, cifs_proto, cifs_proto_count
        );
    }
    Ok(())
}
