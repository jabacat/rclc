use super::structures::DiscoveryRequest;
use super::reqwest;

pub struct DiscoveryServerConfig {
    pub url: String,
}

async fn discover_root(disc_conf: DiscoveryServerConfig) {
    let client = reqwest::Client::new();
    let res = client
        .get(disc_conf.url)
        .send()
        .await
        .expect("Could not await");

    println!("res = {:?}", res);
}

async fn discover_version(disc_conf: DiscoveryServerConfig) -> String {
    let client = reqwest::Client::new();
    let res = client
        .get(disc_conf.url + "/version")
        .send()
        .await
        .expect("Could not await");

    res.text().await.unwrap()
}

async fn discover(disc_conf: DiscoveryServerConfig, disc_request: DiscoveryRequest) {
    let client = reqwest::Client::new();
    let res = client
        .post(disc_conf.url + "/discover")
        .body("Hello!")
        .send()
        .await
        .expect("Could not await");

    println!("res = {:?}", res);
}
