use crate::{
    check_path_exists,
    utils::{get_curr_location, parse_relays_file},
    RELAY_FILE_PATH,
};
use clap::{self, Parser};
use geo::point;
use geo::prelude::*;
use std::{error::Error, path::PathBuf};
use tabled::Tabled;
use tokio::process::Command;

#[derive(Debug, Parser)]
#[command(version,
    about = "mullvad-closest is a really simple (yet efficient) cli app to find the closest mullvad server, for an optimal experience",
    long_about = None
)]
pub struct Args {
    #[arg(short, long, default_value_t = 5000.)]
    pub max_distance: f64,

    #[arg(short, long, default_value = RELAY_FILE_PATH, value_parser = check_path_exists)]
    pub relays_path: PathBuf,
}

#[derive(Debug, Tabled)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    pub fn new(lat: f64, lon: f64) -> Self {
        Location {
            longitude: lon,
            latitude: lat,
        }
    }
    pub fn distance(&self, to: &Location) -> f64 {
        let p1 = point!(x: self.longitude, y: self.latitude);
        let p2 = point!(x: to.longitude, y: to.latitude);
        p1.geodesic_distance(&p2).round() / 1000.0
    }
}

#[derive(Debug, Tabled)]
pub struct Server {
    pub country: String,
    pub city: String,
    #[tabled(inline)]
    pub location: Location,
    pub hostname: String,
    pub ipv4_addr: String,
    pub ipv6_addr: String,
    pub pinged: bool,
    pub latency: f64,
}

impl Server {
    pub async fn get_relevant_servers_from_config(args: Args) -> Vec<Self> {
        let servers = parse_relays_file(args.relays_path);
        let current_location = get_curr_location()
            .await
            .expect("Unable to get the current location");
        servers
            .into_iter()
            .filter(|server| {
                if server.location.distance(&current_location) <= args.max_distance {
                    return true;
                }
                false
            })
            .collect::<Vec<Self>>()
    }

    pub async fn ping(&mut self) -> Result<(), Box<dyn Error>> {
        let out = Command::new("ping")
            .arg(&self.ipv4_addr)
            .arg("-c 2")
            .arg("-q")
            .arg("-n")
            .arg("-s 24")
            .output()
            .await
            .expect("Failed while making a ping!");
        let stdout = String::from_utf8_lossy(&out.stdout);

        // Find the line containing "rtt"
        let rtt_line = stdout
            .lines()
            .find(|line| line.contains("avg"))
            .unwrap_or("");
        let average_time: String = rtt_line
            .split_once('=')
            .unwrap_or(("", ""))
            .1
            .split('/')
            .nth(1)
            .unwrap_or("")
            .to_string();

        if average_time.parse::<f64>().is_ok() {
            self.latency = average_time.parse::<f64>().unwrap();
            self.pinged = true;
        } else {
            self.pinged = false;
        }
        Ok(())
    }
}
