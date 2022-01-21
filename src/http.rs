// use crate::constants;

use log::info;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::net::ToSocketAddrs;
use std::time::Duration;

pub fn build_client(
    insecure_ssl: bool,
    ca_file: &str,
    timeout_sec: u64,
    user_agent: &str,
) -> Result<reqwest::blocking::Client, Box<dyn Error>> {
    let timeout = Duration::from_secs(timeout_sec);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "X-Clacks-Overhead",
        reqwest::header::HeaderValue::from_static("X-Clacks-Overhead"),
    );
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let mut http_client_builder = reqwest::blocking::ClientBuilder::new()
        .user_agent(user_agent)
        .default_headers(headers)
        .timeout(timeout);

    if insecure_ssl {
        http_client_builder = http_client_builder
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true);
    } else if !ca_file.is_empty() {
        let mut ca_buffer = Vec::new();
        let mut fd = match File::open(ca_file) {
            Ok(v) => v,
            Err(e) => bail!("can't open CA file: {}", e),
        };
        if let Err(e) = fd.read_to_end(&mut ca_buffer) {
            bail!("can't read CA data: {}", e);
        }

        let ca_cert = match reqwest::Certificate::from_pem(&ca_buffer) {
            Ok(v) => v,
            Err(e) => bail!("can't decode CA data as PEM format: {}", e),
        };

        http_client_builder = http_client_builder.add_root_certificate(ca_cert);
    }
    let http_client = match http_client_builder.build() {
        Ok(v) => v,
        Err(e) => bail!("can't create HTTP client: {}", e),
    };

    Ok(http_client)
}

pub fn get(
    http_client: &mut reqwest::blocking::Client,
    url: &str,
    user: &str,
    password: &str,
) -> Result<String, Box<dyn Error>> {
    info!("GET {}", &url);

    let response = http_client
        .get(url)
        .basic_auth(user, Some(password))
        .send()?;
    let reply = response.text()?;
    Ok(reply)
}

pub fn socketaddr_from_listen(listen: String) -> Result<std::net::SocketAddr, Box<dyn Error>> {
    let sockaddrs = listen.to_socket_addrs()?;
    let addresses: Vec<_> = sockaddrs.collect();
    if addresses.is_empty() {
        bail!("can't resolve listener address");
    }
    Ok(addresses[0])
}
