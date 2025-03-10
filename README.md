Basic command line Winnipeg Transit bus time checker. 

--- 

### Usage

Intended for personal use, so intended stops are hardcoded over the alternative of finding local stops with GPS data. This is easily modifiable as long as you know which stops and busses you want to take.
To do so, simply add a case to our BusStop FromStr method. These can be visualized like so:

```
"example" => Ok(BusStop {
                alias: s.to_string(),
                stop_number: 00000, // replace zeroes with stop number
                busses_wanted: BusList {
                    busses_wanted: [00].to_vec(), // replace zeroes with busses, or declare an empty vector
                },
            }),
```

Input is prompted at runtime through command line, and should match a case seen alike in the "example" key.

This will also require your own provided `.env`, which requires storage of the WT API key. Format as seen below:

```
    api_key = "YOUR_API_KEY"
```

(it's not that long .. i should also just upload a structure example file but . Later)

---

### Dependancies

[chrono](https://crates.io/crates/chrono)
[dotenv](https://crates.io/crates/dotenv)
[read_input](https://crates.io/crates/read_input)
[reqwest](https://crates.io/crates/reqwest)
[serde / serde_json](https://crates.io/crates/serde)
[tokio](https://crates.io/crates/tokio)