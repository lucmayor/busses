extern crate dotenv;

use chrono::{DateTime, Duration, Local, Timelike};
use dotenv::dotenv;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt, str::FromStr};

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

impl ToString for Bus {
    fn to_string(&self) -> String {
        let mut res = self.alias.clone() + ": ";
        if self.times.len() != 0 {
            for time in &self.times {
                res = res + "\n" + &time.to_string();
            }
        } else {
            res = res + "\nn/a";
        }
        res
    }
}

#[derive(Debug, Deserialize)]
struct TimesTemp {
    scheduled: String,
    estimated: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Copy, Ord, Eq)]
struct Times {
    scheduled: DateTime<Local>,
    estimated: DateTime<Local>,
}

impl PartialOrd for Times {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.estimated.partial_cmp(&other.estimated) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.scheduled.partial_cmp(&other.scheduled)
    }
}

impl ToString for Times {
    fn to_string(&self) -> String {
        let current = Local::now();

        let sched_corrected: Duration = self.scheduled - current;
        let estim_corrected: Duration = self.estimated - current;

        format!(
            "in {0} ({1} minute(s) scheduled — {2}:{3:02})",
            match estim_corrected.num_minutes() {
                0 => format!("{} second(s)", estim_corrected.num_seconds(),),
                _ => format!("{} minute(s)", estim_corrected.num_minutes(),),
            },
            sched_corrected.num_minutes(),
            self.scheduled.hour(),
            self.estimated.minute()
        )
    }
}

// TODO: change datatype to handle blue busses on busses_wanted
struct BusStop {
    alias: String,
    stop_number: i32,
    busses_wanted: BusList,
}

struct BusList {
    busses_wanted: Vec<BusType>,
}

#[derive(Clone)]
enum BusType {
    Integer(u8),
    String(String),
}

impl ToString for BusType {
    fn to_string(&self) -> String {
        if let BusType::Integer(val) = self {
            let temp = val.to_string();
            temp
        } else if let BusType::String(val) = self {
            val.clone()
        } else {
            panic!("unable to convert")
        }
    }
}

impl ToString for BusList {
    fn to_string(&self) -> String {
        let mut res = String::new();

        for bus in self.busses_wanted.clone() {
            res = res + &bus.to_string() + ",";
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
        let mut temp: Vec<BusStop> = Vec::new();

        let busses: Vec<BusStop> = match s {
            "university" | "uni" => {
                temp.push(BusStop::from_str("stafford_south").unwrap());
                temp.push(BusStop::from_str("waverly_south").unwrap());

                temp
            }
            "home_uni" => {
                temp.push(BusStop::from_str("university_one").unwrap());
                temp.push(BusStop::from_str("university_two").unwrap());

                temp
            }
            "home_bus" => {
                temp.push(BusStop::from_str("university_one").unwrap());

                temp
            }
            "ryan" => {
                temp.push(BusStop::from_str("university_blue").unwrap());

                temp
            }
            "late" => {
                temp.push(BusStop::from_str("agriculture_stop").unwrap());

                temp
            }
            _ => panic!("Unimplemented bus case")
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
                    busses_wanted: [BusType::Integer(36)].to_vec(),
                },
            }),
            "corydon_east" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60316,
                busses_wanted: BusList {
                    busses_wanted: [BusType::Integer(18)].to_vec(),
                },
            }),
            "waverly_south" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60306,
                busses_wanted: BusList {
                    busses_wanted: [BusType::Integer(78)].to_vec(),
                },
            }),
            "university_one" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60674,
                busses_wanted: BusList {
                    busses_wanted: [BusType::Integer(36)].to_vec(),
                },
            }),
            "university_two" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60673,
                busses_wanted: BusList {
                    busses_wanted: [BusType::Integer(78)].to_vec(),
                },
            }),
            "agriculture_stop" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60105,
                busses_wanted: BusList {
                    busses_wanted: [
                        BusType::Integer(36),
                        BusType::Integer(78),
                        BusType::String("BLUE".to_owned()),
                    ]
                    .to_vec(),
                },
            }),
            "university_blue" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 60675,
                busses_wanted: BusList {
                    busses_wanted: [BusType::String("BLUE".to_owned())].to_vec(),
                },
            }),
            "downtown_rwb_west" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 10617,
                busses_wanted: BusList {
                    busses_wanted: [BusType::Integer(18)].to_vec(),
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
            _ => {
                tokio::task::spawn_blocking(move || results_loop());
            }
        },
        Err(e) => panic!("Error in first read-in: {:?}", e),
    }
}

// main loop, spawn off async so we don't create concurrent threads
fn results_loop() {
    loop {
        let res = get_results().unwrap();
        match res {
            'q' => {
                println!("Quitting program.");
                break;
            }
            'c' => {}
            _ => panic!("idk what just happened"),
        }
    }
}

fn get_results() -> Result<char, Box<dyn std::error::Error>> {
    dotenv().ok();
    let blocking_client = reqwest::blocking::Client::new();

    let input: String = input().get();

    if input == "q" {
        return Ok('q');
    }

    let to_search = match StopCollection::from_str(&input) {
        Ok(stops) => stops,
        Err(_e) => panic!("Couldn't build stops"),
    };

    println!("For collection {:?}", to_search.alias);

    let mut final_list: Vec<Bus> = Vec::new();

    for stops in to_search.stops {
        let mut param: HashMap<&str, &str> = HashMap::new();
        let api_key = &std::env::var("api_key").expect("api key of doom");
        param.insert("api-key", api_key);
        param.insert("max-results-per-route", "3"); // seems to max out at 3

        let routes = &stops.busses_wanted.to_string();
        if routes.len() != 0 {
            param.insert("route", routes);
        }

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

        if routes.len() == 0 {
            println!("No busses found on route \'{0}\'.", stops.alias);
        }

        for route in routes {
            let name = match route.get("route").and_then(|k| k.get("key")) {
                Some(n) => {
                    if let Some(as_num) = n.as_i64() {
                        &*as_num.to_string()
                    } else if let Some(as_str) = n.as_str() {
                        as_str
                    } else {
                        "n/a"
                    }
                }
                None => "n/a",
            };

            if let Some(stops) = route.get("scheduled-stops").and_then(|s| s.as_array()) {
                let mut result: Vec<Times> = Vec::new();

                for stop in stops {
                    if let Some(stop_time) = stop.get("times") {
                        if let Some(arrival_key) = stop_time.get("departure") {
                            // this is kinda shit and we can probably find a way to manipulate the string
                            // without the interim struct / handler
                            if let Ok(mut interim) =
                                serde_json::from_value::<TimesTemp>(arrival_key.clone())
                            {
                                interim.estimated.push_str("-05:00");
                                interim.scheduled.push_str("-05:00");
                                let updated_times: Times = Times {
                                    estimated: interim
                                        .estimated
                                        .parse::<DateTime<Local>>()
                                        .unwrap(),
                                    scheduled: interim
                                        .scheduled
                                        .parse::<DateTime<Local>>()
                                        .unwrap(),
                                };
                                result.push(updated_times);
                            }
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

    let bus: Vec<(String, Times)> = group_busses(final_list);
    for item in bus {
        println!("{0}: {1}", item.0, item.1.to_string());
    }

    Ok('c')
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

fn group_busses(bus_list: Vec<Bus>) -> Vec<(String, Times)> {
    let mut out_list: Vec<(String, Times)> = Vec::new();

    for item in bus_list {
        for times in item.times {
            out_list.push((item.alias.clone(), times))
        }
    }

    out_list.sort_by_key(|k| k.1);
    out_list
}