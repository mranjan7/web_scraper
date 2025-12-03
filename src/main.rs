use chrono::Utc;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use remote_job_scraper::jobs::Job;
use serde_json::Value;
use chrono::TimeZone;

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
    let mut writer = csv::Writer::from_path("remote_jobs.csv")?;
    for job in &jobs {
            writer.serialize(job)?;
        }
    Ok(())
}
async fn scrape_remoteok(seen: &mut HashSet<String>) -> Result<Vec<Job>, Box<dyn std::error::Error>>{
        let mut result = Vec::new();
        let jobs:Vec<Value> = reqwest::Client::builder().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0")
                                .build()?
                                .get("https://remoteok.com/api")
                                .send()
                                .await?
                                .json()
                                .await?;
            let tag_index = 7;
            for job in jobs.iter().skip(1).take(10){
                    let title = job["position"].as_str().unwrap_or("NA").to_string();
                    let company = job["company"].as_str().unwrap_or("NA").to_string();
                    let location = job["location"].as_str().unwrap_or("NA").to_string();
                    let salary_min = job["salary_min"].as_str().unwrap_or("NA").to_string();
                    let salary_max = job["salary_max"].as_str().unwrap_or("NA").to_string();
                    let tags:Vec<String> = job["tags"]
                    .as_array()
                    .and_then(|row| row.get(tag_index))
                    .and_then(|v| v.as_array())
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                    let url = job["apply_url"].as_str().unwrap_or("NA").to_string();
                    let posted = Utc.timestamp_opt(job["epoch"].as_i64().unwrap(),0)
                                    .single()
                                    .unwrap()
                                    .format("%Y-%m-%d")
                                    .to_string();


                    let my_job = Job{
                           title: title.clone(),
                           company: company.clone(),
                           location: location.clone(),
                           salary_min: salary_min.clone(),
                           salary_max: salary_max.clone(),
                           tags: tags.join(" | ").clone(),
                           url: url.clone(),
                           posted:posted.clone(),
                        };
                    result.push(my_job);
                    println!("{}",title);
                }
           Ok(result)
    }
async fn scrape_weworkremotely(seen: &mut HashSet<String>) -> Result<Vec<Job>, Box<dyn std::error::Error>>{
            Ok(Vec::new())
         }
