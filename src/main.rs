use chrono::Utc;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use remote_job_scraper::jobs::Job;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut jobs = Vec::new();
    let mut seen_urls = HashSet::new();
    println!("Scarping remote ok .......");
    let mut result = scrape_remoteok(&mut seen_urls).await?;
    jobs.extend(result);
    thread::sleep(Duration::from_secs(2));
    println!("Scarping we work remotely.................");
    result = scrape_weworkremotely(&mut seen_urls).await?;
    jobs.extend(result);
    let json = serde_json::to_string_pretty(&jobs)?;
    let mut file = File::create("remote_jobs.json").expect("could not create file");
    file.write_all(json.as_bytes())?;
    Ok(())
}
async fn scrape_remoteok(seen: &mut HashSet<String>) -> Result<Vec<Job>, Box<dyn std::error::Error>>{
    let body = reqwest::get("https://remoteok.com/remote-jobs").await?.text().await?;
    println!("{body}");
    Ok(Vec::new())
    }
async fn scrape_weworkremotely(seen: &mut HashSet<String>) -> Result<Vec<Job>, Box<dyn std::error::Error>>{
            Ok(Vec::new())
         }
