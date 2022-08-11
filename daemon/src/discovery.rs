use super::reqwest;
use super::structures::DiscoveryRequest;
use anyhow::Result;

pub struct DiscoveryServerConfig {
    pub url: String,
}

pub async fn discover_root(disc_conf: DiscoveryServerConfig) -> Result<String> {
    let client = reqwest::Client::new();
    let res = client
        .get(disc_conf.url)
        .send()
        .await?;

    Ok(res.text().await?)
}

pub async fn discover_version(disc_conf: DiscoveryServerConfig) -> Result<String> {
    let client = reqwest::Client::new();
    let res = client
        .get(disc_conf.url + "/version")
        .send()
        .await?;

    Ok(res.text().await?)
}

pub async fn discover(disc_conf: DiscoveryServerConfig, disc_request: DiscoveryRequest) {
    let client = reqwest::Client::new();
    let res = client
        .post(disc_conf.url + "/discover")
        .body("Hello!")
        .send()
        .await
        .expect("Could not discover");

    println!("res = {:?}", res);
}
