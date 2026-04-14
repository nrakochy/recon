use error::Error;
use rayon::prelude::*;
use reqwest::blocking::Client;
use reqwest::redirect;
use std::time::Duration;

mod constants;
mod model;
mod ports;
mod subdomains;

pub fn scan(target: &str) -> Result<(), error::Error> {
    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    // we use a custom threadpool to improve speed
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    // pool.install is required to use our custom threadpool, instead of rayon's default one
    pool.install(|| {
        let scan_result: Vec<model::Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("    {}", port.port);
            }

            println!();
        }
    });

    Ok(())
}
