pub mod jobs{

use serde::Serialize;
#[derive(Serialize, Debug, Clone)]
pub struct Job {
       title: String,
       company: String,
       location: String,
       salary: String,
       tags: Vec<String>,
       url: String,
       source: String,
       posted: String,
   }
}