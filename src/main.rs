extern crate dotenv;

use chrono::{DateTime, FixedOffset, format::Fixed};
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt, str::FromStr};
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    status: HashMap<String, String>,
}

#[derive(Debug)]
enum LocError {
    Other,
}

struct Bus {
    alias: String,
    times: Vec<Times>,
}

#[derive(Debug, Deserialize)]
struct Times {
    scheduled: DateTime<FixedOffset>,
    estimated: DateTime<FixedOffset>,
}

// TODO: change datatype to handle blue busses on busses_wanted
struct BusStop {
    alias: String,
    stop_number: i32,
    busses_wanted: BusList,
}

struct BusList {
    busses_wanted: Vec<u8>,
}

impl ToString for BusList {
    fn to_string(&self) -> String {
        let mut res = String::new();

        for bus in self.busses_wanted.clone() {
            res = res + &bus.to_string();
        }
        
        res
    }
}

struct StopCollection {
    alias: String,
    stops: Vec<BusStop>,
}

impl std::str::FromStr for StopCollection {
    type Err = LocError;

    fn from_str(s: &str) -> Result<Self, LocError> {
        let busses: Vec<BusStop> = match s {
            "university" => {
                let mut temp: Vec<BusStop> = Vec::new();

                temp.push(BusStop::from_str("stafford_south").unwrap());
                temp.push(BusStop::from_str("waverly_south").unwrap());

                temp
            }
            "home_uni" => {
                let mut temp: Vec<BusStop> = Vec::new();

                temp.push(BusStop::from_str("university_one").unwrap());
                temp.push(BusStop::from_str("university_two").unwrap());

                temp
            }
            _ => todo!(),
        };

        Ok(StopCollection {
            alias: s.to_string(),
            stops: busses,
        })
    }
}

impl std::str::FromStr for BusStop {
    type Err = LocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stafford_south" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 10102,
                busses_wanted: BusList {
                    busses_wanted: [36].to_vec(),
                },
            }),
            "corydon_east" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60316,
                busses_wanted: BusList {
                    busses_wanted: [18].to_vec(),
                },
            }),
            "waverly_south" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60306,
                busses_wanted: BusList {
                    busses_wanted: [78].to_vec(),
                },
            }),
            "university_one" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60674,
                busses_wanted: BusList {
                    busses_wanted: [36].to_vec(),
                },
            }),
            "university_two" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60673,
                busses_wanted: BusList {
                    busses_wanted: [78].to_vec(),
                },
            }),
            "agriculture_stop" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60105,
                busses_wanted: BusList {
                    busses_wanted: [36, 78].to_vec(),
                },
            }),
            "university_blue" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60675,
                busses_wanted: BusList {
                    busses_wanted: [0].to_vec(),
                },
            }),
            "downtown_rwb_west" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 10617,
                busses_wanted: BusList {
                    busses_wanted: [18].to_vec(),
                },
            }),
            _ => Err(LocError::Other),
        }
    }
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
            // THIS IS UNSAFE AS FUCK !!!
            // THIS IS UNSAFE AS FUCK !!!
            // THIS IS UNSAFE AS FUCK !!!
            // THIS IS UNSAFE AS FUCK !!!
            // This will crash your computer in like 30 seconds don't use this !!!
            _ => loop {
                tokio::task::spawn_blocking(move || get_results().unwrap());
            },
        },
        Err(e) => panic!("Error in first read-in: {:?}", e),
    }
}

fn get_results() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let blocking_client = reqwest::blocking::Client::new();

    let input: String = input().get();
    let to_search = match StopCollection::from_str(&input) {
        Ok(stops) => stops,
        Err(_e) => panic!("Couldn't build stops"),
    };

    println!("For collection {:?}", to_search.alias);

    for stops in to_search.stops {
        println!("For stop {:?}", stops.alias);

        let mut param: HashMap<&str, &str> = HashMap::new();
        let api_key = &std::env::var("api_key").expect("api key of doom");
        param.insert("api-key", api_key);
        param.insert("max-results-per-route", "3"); // seems to max out at 3

        let routes = &stops.busses_wanted.to_string();
        param.insert("routes", routes);

        let url = format!(
            "https://api.winnipegtransit.com/v3/stops/{0}/schedule.json",
            stops.stop_number
        );
        let res = blocking_client
            .get(url)
            .query(&param)
            .send()?
            .text()
            .expect("Couldn't get response from WT");

        let v: Value = serde_json::from_str(&res)?;
        let routes = match v
            .get("stop-schedule")
            .and_then(|a| a.get("route-schedules"))
            .and_then(|b| b.as_array())
        {
            Some(b) => b,
            None => &vec![],
        };

        let mut final_list: Vec<Bus> = Vec::new();

        for route in routes {
            let name = match route.get("key") {
                Some(n) => match n.as_str() {
                    Some(bus) => bus,
                    None => "n/a",
                },
                None => "n/a",
            };

            if let Some(stops) = route.get("scheduled-stops").and_then(|s| s.as_array()) {
                let mut result: Vec<Times> = Vec::new();

                for stop in stops {
                    if let Some(stop_time) = stop.get("times") {
                        // there's probably gonna be some timezone bullshit with this
                        // need to figure out how to set the timezone for item
                        if let Ok(times) = serde_json::from_value(stop_time.clone()) {
                            result.push(times);
                        }
                    }
                }
                final_list.push(Bus {
                    alias: name.to_owned(),
                    times: result,
                })
            }
        }
    }

    Ok(())
}

async fn validate() -> Result<Status, reqwest::Error> {
    dotenv().ok();

    let mut param: HashMap<&str, &str> = HashMap::new();
    let api_key = &std::env::var("api_key").expect("api key of doom");
    param.insert("api-key", api_key);

    let client = reqwest::Client::new();

    client
        .post("https://api.winnipegtransit.com/v3/statuses/schedule.json")
        .query(&param)
        .send()
        .await?
        .json::<Status>()
        .await
}
