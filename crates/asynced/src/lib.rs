mod ports;
mod subdomains;

use crate::ports::scan_ports;
use common::model::Subdomain;
use error::Error;
use futures::{StreamExt, stream};
use reqwest::Client;
use std::time::{Duration, Instant};

pub async fn scan(target: &str) -> Result<(), Error> {
    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;

    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let _scan_start = Instant::now();

    let subdomains = subdomains::enumerate(&http_client, target).await?;

    let _scan_result: Vec<Subdomain> = stream::iter(subdomains.into_iter())
        .map(|subdomain| scan_ports(ports_concurrency, subdomain))
        .buffer_unordered(subdomains_concurrency)
        .collect()
        .await;

    Ok(())
}
