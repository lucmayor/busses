use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    status: HashMap<String, String>,
}

#[derive(Debug)]
enum LocError {
    Other,
}

impl std::error::Error for LocError {}

impl fmt::Display for LocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Something fucked up"),
        }
    }
}

#[tokio::main]
async fn main() {
    match validate().await {
        Ok(stat) => match stat.status.get("value").unwrap().as_str() {
            "esp-1" | "esp-2" | "esp-3" => panic!("Presently not in service"),
            _ => loop {
                tokio::task::spawn_blocking(move || get_results().unwrap());
            },
        },
        Err(e) => panic!("Error in first read-in: {:?}", e),
    }
}

fn get_results() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get("https://api.winnipegtransit.com/v3/statuses/schedule.json")?;

    todo!()
}

async fn validate() -> Result<Status, reqwest::Error> {
    let mut param: HashMap<&str, &str> = HashMap::new();

    param.insert("api-key", "qZ_UkLdcaB4C1KDKXgeq");

    let client = reqwest::Client::new();

    client
        .post("https://api.winnipegtransit.com/v3/statuses/schedule.json")
        .query(&param)
        .send()
        .await?
        .json::<Status>()
        .await
}
