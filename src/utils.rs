use crate::structs::{Location, Server};
use reqwest;
use serde_json::Value;
use std::fs::read_to_string;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub const RELAY_FILE_PATH: &'static str = "/Library/Caches/mullvad-vpn/relays.json";

#[cfg(target_os = "windows")]
pub const RELAY_FILE_PATH: &'static str = "C:/ProgramData/Mullvad VPN/cache/relays.json";

#[cfg(target_os = "linux")]
pub const RELAY_FILE_PATH: &'static str = "/var/cache/mullvad-vpn/relays.json";

pub fn check_path_exists(path: &str) -> Result<PathBuf, String> {
    let parsed_path = PathBuf::from(path);
    if parsed_path.try_exists().unwrap() {
        return Ok(parsed_path);
    }
    Err(format!("Provided path {} does not exist", path))
}

pub fn parse_relays_file<'a>(file_path: PathBuf) -> Vec<Server> {
    let content = read_to_string(file_path).unwrap();
    let v: Value = serde_json::from_str(&content).unwrap();
    v["countries"]
        .as_array()
        .unwrap()
        .into_iter()
        .flat_map(|country| {
            country["cities"]
                .as_array()
                .unwrap()
                .into_iter()
                .flat_map(|city| {
                    city["relays"].as_array().unwrap().into_iter().map(|relay| {
                        Server {
                            country: country["name"].as_str().unwrap().to_string(), // Directly converting to string was giving a weird result, need to investigate
                            city: city["name"].as_str().unwrap().to_string(),
                            location: Location {
                                latitude: relay["location"]["latitude"].as_f64().unwrap(),
                                longitude: relay["location"]["longitude"].as_f64().unwrap(),
                            },
                            hostname: relay["hostname"].as_str().unwrap().to_string(),
                            ipv4_addr: relay["ipv4_addr_in"].as_str().unwrap().to_string(),
                            ipv6_addr: relay["ipv6_addr_in"].as_str().unwrap_or("").to_string(),
                            pinged: false,
                            latency: f64::MAX,
                        }
                    })
                })
        })
        .collect::<Vec<Server>>()
}

pub async fn get_curr_location() -> Result<Location, String> {
    let resp = reqwest::get("http://ip-api.com/json/").await;
    if resp.as_ref().is_ok_and(|r| r.status().is_success()) {
        let loc_info: Value = serde_json::from_str(&resp.unwrap().text().await.unwrap()).unwrap();
        return Ok(Location {
            latitude: loc_info["lat"].as_f64().unwrap(),
            longitude: loc_info["lon"].as_f64().unwrap(),
        });
    }
    Err("Unable to ping the GepLocation server".to_string())
}
