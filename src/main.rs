use chrono::Utc;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use remote_job_scraper::jobs::Job;
use serde_json::Value;


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
        let jobs:Vec<Value> = reqwest::Client::builder().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0")
                                .build()?
                                .get("https://remoteok.com/api")
                                .send()
                                .await?
                                .json()
                                .await?;
            for job in jobs.iter().skip(1).take(10){
                    let title = job["position"].as_str().unwrap_or("NA");
                    println!("{}",title);
                }
           Ok(())
    }
async fn scrape_weworkremotely(seen: &mut HashSet<String>) -> Result<Vec<Job>, Box<dyn std::error::Error>>{
            Ok(Vec::new())
         }
