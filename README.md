# Remote Job Scraper

A Rust-based command-line application that scrapes remote job listings from [RemoteOK](https://remoteok.com/) and saves them as both JSON and CSV files.

## Features

- Fetches up to 500 remote jobs from RemoteOK.
- Extracts essential job details:
  - Job title
  - Company
  - Location
  - Salary range
  - Tags
  - Application URL
  - Date posted
- Saves data in:
  - `remote_jobs.json` (pretty-printed JSON)
  - `remote_jobs.csv` (CSV format)

## Prerequisites

- Rust (1.70+ recommended)
- Cargo package manager

## Dependencies

This project uses the following Rust crates:

- `tokio` – For asynchronous runtime.
- `reqwest` – For making HTTP requests.
- `serde` and `serde_json` – For JSON serialization/deserialization.
- `chrono` – For date and time handling.
- `csv` – For writing CSV files.
- `remote_job_scraper` – Custom module for Job struct definition.

Make sure to include these dependencies in your `Cargo.toml`.

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd <repository-folder>

2. Build the ptoject
cargo build --release

3. Run the scraper
cargo run

Upon successful execution following two file will be formed in root project directory:
remote_jobs.json will contain the job listings in JSON format.
remote_jobs.csv will contain the job listings in CSV format.









