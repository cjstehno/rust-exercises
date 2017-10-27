extern crate console;
#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate serde_json;

use serde_json::Value;
use clap::App;
use console::Term;
use std::process::exit;

const BASE_URL: &'static str = "http://api.openweathermap.org/data/2.5/weather";

fn main() {
    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let term = Term::stdout();

    let zip_code = required_input(&term, matches.value_of("zip"), "ZIP code");
    let api_key = required_input(&term, matches.value_of("key"), "API key");

    let mut resp = reqwest::get(&format!("{}?zip={}&APPID={}", BASE_URL, zip_code, api_key)).unwrap();

    if resp.status().is_success(){
        let json: Value = resp.json().unwrap();

        println!(
            "{}\"{}\" {:2.0}℉ (high: {:2.0}℉, low: {:2.0}℉) {:3.1} mph winds.",
            select_icon(json["weather"][0]["icon"].as_str().unwrap()),
            json["weather"][0]["description"].as_str().unwrap(),
            temp_k_to_f(json["main"]["temp"].as_f64().unwrap()),
            temp_k_to_f(json["main"]["temp_min"].as_f64().unwrap()),
            temp_k_to_f(json["main"]["temp_max"].as_f64().unwrap()),
            json["wind"]["speed"].as_f64().unwrap()
        );

    } else {
        println!("🌡️ Unable to contact weather server.")
    }
}

fn select_icon(icon_id: &str) -> &str {
    match icon_id {
        "01d" => "☀️",
        "01n" => "🌕",
        "02d" | "02n" => "⛅",
        "03d" | "03n" => "☁️",
        "04d" | "04n" => "☁",
        "09d" | "09n" => "🌧️",
        "10d" | "10n" => "🌦️",
        "11d" | "11n" => "⛈️",
        "13d" | "13n" => "🌨️",
        "50d" | "50n" => "🌫️",
        _ => "🌡️"
    }
}

fn required_input(term: &Term, arg: Option<&str>, label: &str) -> String {
    match arg {
        Some(value) => String::from(value),
        None => {
            term.write_str(&format!("What is your {}? ", label)).unwrap();

            match term.read_line() {
                Ok(line) => line,
                Err(_) => {
                    term.write_line(&format!("A valid {} must be provided.", label)).unwrap();
                    exit(0);
                }
            }
        }
    }
}

fn temp_k_to_f(kelvin: f64) -> f64 {
    kelvin * 9.0 / 5.0 - 459.67
}