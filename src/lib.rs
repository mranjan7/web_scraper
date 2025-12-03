pub mod jobs{

use serde::Serialize;
#[derive(Serialize, Debug, Clone)]
pub struct Job {
       pub title: String,
       pub company: String,
       pub location: String,
       pub salary_min: String,
       pub salary_max: String,
       pub tags: String,
       pub url: String,
       pub posted: String,
   }
}