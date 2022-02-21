use crate::config;
use crate::constants;
use crate::exporter;
use crate::http;

use log::debug;
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
    client_ip: bool,
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
    let mut volumes = HashMap::<String, i64>::new();
    let mut users = HashMap::<String, i64>::new();
    let mut mapped_users = HashMap::<String, i64>::new();
    let mut open_shares: i64 = 0;
    let mut open_files: i64 = 0;
    let mut open_others: i64 = 0;
    let mut smb_signing_on: i64 = 0;
    let mut smb_signing_off: i64 = 0;
    let mut connections: i64 = 0;
    let mut large_mtu_on: i64 = 0;
    let mut large_mtu_off: i64 = 0;

    let mut protocols = HashMap::<String, i64>::new();
    for p in constants::CIFS_PROTOCOL_LIST {
        protocols.insert(p.to_string(), 0);
    }

    let mut smb_encryptions = HashMap::<String, i64>::new();
    for e in constants::CIFS_SMB_ENCRYPTION_LIST {
        smb_encryptions.insert(e.to_string(), 0);
    }

    let mut continuous_availabilities = HashMap::<String, i64>::new();
    for c in constants::CIFS_CONTINUOUS_AVAILABILITY_LIST {
        continuous_availabilities.insert(c.to_string(), 0);
    }

    let mut authentications = HashMap::<String, i64>::new();
    for a in constants::CIFS_AUTHENTICATION_LIST {
        authentications.insert(a.to_string(), 0);
    }

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
        if client_ip {
            *clients.entry(cifs.client_ip).or_insert(0) += 1;
        }
        if cifs.large_mtu {
            large_mtu_on += 1;
        } else {
            large_mtu_off += 1;
        }
    }

    for (proto, proto_cnt) in protocols {
        debug!(
            "Updating metrics for cifs protocol -> {} {} {}",
            filer.name, proto, proto_cnt
        );
        exporter::CIFS_PROTOCOLS
            .with_label_values(&[&filer.name, &proto])
            .set(proto_cnt);
    }

    for (enc, enc_count) in smb_encryptions {
        debug!(
            "Updating metrics for cifs smb_encryption -> {} {} {}",
            filer.name, enc, enc_count
        );
        exporter::CIFS_SMB_ENCRYPTION
            .with_label_values(&[&filer.name, &enc])
            .set(enc_count);
    }

    for (ca, ca_cnt) in continuous_availabilities {
        debug!(
            "Updating metrics for cifs continuous_availability -> {} {} {}",
            filer.name, ca, ca_cnt
        );
        exporter::CIFS_CONTINUOUS_AVAILABILITY
            .with_label_values(&[&filer.name, &ca])
            .set(ca_cnt);
    }

    debug!(
        "Updating metrics for cifs open_files -> {} {}",
        filer.name, open_files
    );
    exporter::CIFS_OPEN_FILES
        .with_label_values(&[&filer.name])
        .set(open_files);

    debug!(
        "Updating metrics for cifs open_shares -> {} {}",
        filer.name, open_shares
    );
    exporter::CIFS_OPEN_SHARES
        .with_label_values(&[&filer.name])
        .set(open_shares);

    debug!(
        "Updating metrics for cifs open_other -> {} {}",
        filer.name, open_others
    );
    exporter::CIFS_OPEN_OTHER
        .with_label_values(&[&filer.name])
        .set(open_others);

    for (auth, auth_cnt) in authentications {
        debug!(
            "Updating metrics for cifs authentication -> {} {} {}",
            filer.name, auth, auth_cnt
        );
        exporter::CIFS_AUTHENTICATION
            .with_label_values(&[&filer.name, &auth])
            .set(auth_cnt);
    }

    debug!(
        "Updating metrics for cifs smb_signing -> {} {} {}",
        filer.name, "on", smb_signing_on
    );
    exporter::CIFS_SMB_SIGNING
        .with_label_values(&[&filer.name, "on"])
        .set(smb_signing_on);

    debug!(
        "Updating metrics for cifs smb_signing -> {} {} {}",
        filer.name, "off", smb_signing_off
    );
    exporter::CIFS_SMB_SIGNING
        .with_label_values(&[&filer.name, "off"])
        .set(smb_signing_off);

    if user {
        for (u, u_cnt) in users {
            debug!(
                "Updating metrics for cifs users -> {} {} {}",
                filer.name, u, u_cnt
            );
            exporter::CIFS_USER
                .with_label_values(&[&filer.name, &u])
                .set(u_cnt);
        }
    }

    if mapped_user {
        for (u, u_cnt) in mapped_users {
            debug!(
                "Updating metrics for cifs mapped_unix_users -> {} {} {}",
                filer.name, u, u_cnt
            );
            exporter::CIFS_MAPPED_UNIX_USER
                .with_label_values(&[&filer.name, &u])
                .set(u_cnt);
        }
    }

    if client_ip {
        for (cli, cli_cnt) in clients {
            debug!(
                "Updating metrics for cifs client_ip -> {} {} {}",
                filer.name, cli, cli_cnt
            );
            exporter::CIFS_CLIENT
                .with_label_values(&[&filer.name, &cli])
                .set(cli_cnt);
        }
    }

    for (vol, vol_cnt) in volumes {
        debug!(
            "Updating metrics for cifs volumes -> {} {} {}",
            filer.name, vol, vol_cnt
        );
        exporter::CIFS_VOLUME
            .with_label_values(&[&filer.name, &vol])
            .set(vol_cnt);
    }

    debug!(
        "Updating metrics for cifs large_mtu -> {} {} {}",
        filer.name, "on", large_mtu_on
    );
    exporter::CIFS_LARGE_MTU
        .with_label_values(&[&filer.name, "on"])
        .set(large_mtu_on);

    debug!(
        "Updating metrics for cifs large_mtu -> {} {} {}",
        filer.name, "off", large_mtu_off
    );
    exporter::CIFS_LARGE_MTU
        .with_label_values(&[&filer.name, "off"])
        .set(large_mtu_off);

    debug!(
        "Updating metrics for cifs connection_count -> {} {}",
        filer.name, connections
    );
    exporter::CIFS_CONNECTION
        .with_label_values(&[&filer.name])
        .set(connections);

    Ok(())
}
