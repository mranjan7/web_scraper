pub mod jobs{

use serde::Serialize;
#[derive(Serialize, Debug, Clone)]
pub struct Job {
       pub title: String,
   }
}