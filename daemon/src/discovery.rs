use super::reqwest;
use super::{DiscoveryRequest, DiscoveryResponse};
use crate::contact::Contact;
use serde_json::Result as ParseResult;
use anyhow::Result;
use log::debug;

#[derive(Clone)]
pub struct DiscoveryServerConfig {
    pub url: String,
}

pub async fn discover_root(disc_conf: DiscoveryServerConfig) -> Result<String> {
    let client = reqwest::Client::new();
    let res = client.get(disc_conf.url).send().await?;

    Ok(res.text().await?)
}

pub async fn discover_version(disc_conf: DiscoveryServerConfig) -> Result<String> {
    let client = reqwest::Client::new();
    let res = client.get(disc_conf.url + "/version").send().await?;

    Ok(res.text().await?)
}

pub async fn discover(
    disc_conf: DiscoveryServerConfig,
    disc_request: DiscoveryRequest,
) -> Result<DiscoveryResponse> {
    let client = reqwest::Client::new();
    let url = disc_conf.url + "/discover";
    debug!("{}", url);

    let res = client.post(url).json(&disc_request).send().await?;
    let text = res.text().await?;

    Ok(serde_json::from_str(&text).unwrap())
}
