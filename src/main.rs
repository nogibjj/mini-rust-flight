use clap::{App, Arg};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct FlightStatus {
    flight: FlightInfo,
    departure: FlightSchedule,
    arrival: FlightSchedule,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct FlightInfo {
    iata: String,
    icao: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct FlightSchedule {
    iata: String,
    icao: String,
    scheduled: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Flight Status Checker")
        .version("1.0")
        .author("Minghui Zhu <zhuminghui17@gmail.com>")
        .about("Check flight status")
        .arg(
            Arg::with_name("flight_code")
                .short('f')
                .long("flight")
                .value_name("FLIGHT_CODE")
                .help("Flight code (e.g., AA123)")
                .required(true),
        )
        .get_matches();

    let flight_code = matches.value_of("flight_code").unwrap();

    let api_key = "d2b531fd31d95be62dedea1e1070eb70";
    let url = format!(
        "http://api.aviationstack.com/v1/flights?access_key={}&flight_iata={}",
        api_key, flight_code
    );

    let client = Client::new();
    let response = client.get(&url).send().await?.json::<serde_json::Value>().await?;

    if let Some(flight_data) = response["data"].as_array().and_then(|arr| arr.get(0)) {
        let flight_status: FlightStatus = serde_json::from_value(flight_data.clone())?;
        println!("{:#?}", flight_status);
    } else {
        println!("No flight found with the provided flight code");
    }

    Ok(())
}
