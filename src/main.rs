use chrono::Utc;
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Debug, Clone)]
struct Job {
    title: String,
    company: String,
    location: String,
    salary: String,
    tags: Vec<String>,
    url: String,
    source: String,
    posted: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut jobs = Vec::new();
    let mut seen_urls = HashSet::new();
    println!("Scarping remote ok .......");
    jobs.extend(scrape_remoteok(&mut seen_urls).await();
    thread::sleep(Duration::from_secs(2));
    println!("Scarping we work remotely.................");
    jobs.extend(scrape_weworkremotely(&mut seen_urls);

    let json = serde_json::to_string_pretty(&jobs)?;
    let mut file = File::create("remote_jobs.json");
    file.write_all(json.as_bytes())?;
}
