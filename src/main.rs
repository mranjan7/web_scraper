use chrono::Utc;
use chrono::TimeZone;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use remote_job_scraper::jobs::Job;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut jobs = Vec::new();
    println!("iremote ok .......");
    let mut result = scrape_remoteok().await?;
    jobs.extend(result);
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Scraping completed.................");
    let json = serde_json::to_string_pretty(&jobs)?;
    let mut file = File::create("remote_jobs.json").expect("could not create file");
    file.write_all(json.as_bytes())?;
    let mut writer = csv::Writer::from_path("remote_jobs.csv")?;
    for job in &jobs {
            writer.serialize(job)?;
        }
    Ok(())
}
async fn scrape_remoteok() -> Result<Vec<Job>, Box<dyn std::error::Error>>{
        let mut result = Vec::new();
        let jobs:Vec<Value> = reqwest::Client::builder().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0")
                                .build()?
                                .get("https://remoteok.com/api")
                                .query(&[("limit","500")])
                                .send()
                                .await?
                                .json()
                                .await?;
            let tag_index = 7;
            for job in jobs.iter().skip(1){
                    let title = job["position"].as_str().unwrap_or("NA").to_string();
                    let company = job["company"].as_str().unwrap_or("NA").to_string();
                    let location = job["location"].as_str().unwrap_or("NA").to_string();
                    let salary_min = job["salary_min"].as_str().unwrap_or("NA").to_string();
                    let salary_max = job["salary_max"].as_str().unwrap_or("NA").to_string();
                    let tags = job["tags"]
                    .as_array()
                    .map_or(Vec::new(), |arr| {
                    arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
                    });
                    let url = job["apply_url"].as_str().unwrap_or("NA").to_string();
                    let posted =  job["epoch"].as_i64()
                                    .and_then(|epoch| Utc.timestamp_opt(epoch,0).single())
                                    .map(|dt| dt.format("%Y-%m-%d").to_string())
                                    .unwrap_or_else(|| "NA".to_string());


                    let my_job = Job{
                           title: title,
                           company: company,
                           location: location,
                           salary_min: salary_min,
                           salary_max: salary_max,
                           tags: tags.join(" | "),
                           url: url,
                           posted:posted,
                        };
                    result.push(my_job);
                }
           Ok(result)
    }
