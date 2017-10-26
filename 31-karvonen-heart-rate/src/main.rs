extern crate console;
extern crate clap;

use clap::{Arg, App, SubCommand};
use console::Term;
use std::process::exit;

fn main() {
    let matches = App::new("karvonen-heart-rate")
        .version("0.0.1")
        .author("Christopher J. Stehno <chris@stehno.com>")
        .about("Calculates your target heart rate given age and resting heart rate.")
        .arg(Arg::with_name("age")
            .short("a")
            .long("age")
            .value_name("AGE")
            .help("Specifies the age.")
            .takes_value(true))
        .arg(Arg::with_name("resting")
            .short("r")
            .long("resting")
            .value_name("RATE")
            .help("Specifies the resting heart rate.")
            .takes_value(true))
        .get_matches();

    let term = Term::stdout();

    let mut age: u16 = 0;

    if let Some(value) = matches.value_of("age") {
        match value.parse::<u16>() {
            Ok(num) => age = num,
            Err(_) => {
                term.write_line("A valid age must be provided.").unwrap();
                exit(0);
            }
        }
    } else {
        term.write_str("How old are you? ").unwrap();

        age = parse_number_input(&term, "age");
    }

    let mut resting_heart_rate: u16 = 0;

    if let Some(value) = matches.value_of("resting") {
        match value.parse::<u16>() {
            Ok(num) => resting_heart_rate = num,
            Err(_) => {
                term.write_line("A valid resting heart rate must be provided.").unwrap();
                exit(0);
            }
        }
    } else {
        term.write_str("What is your resting heart rate? ").unwrap();

        resting_heart_rate = parse_number_input(&term, "resting heart rate");
    }

    term.write_line(&format!("\nResting Pulse: {}  Age: {}\n", resting_heart_rate, age)).unwrap();

    term.write_line("Intensity | Target").unwrap();
    term.write_line("-------------------").unwrap();

    let mut intensity: u16 = 50;
    while intensity < 100 {
        let target = calculate_target_rate(age, resting_heart_rate, intensity);
        term.write_line(&format!("{}%       | {} bpm", intensity, target)).unwrap();

        intensity += 5;
    }
}

fn parse_number_input(term: &Term, label: &str) -> u16 {
    if let Ok(line) = term.read_line() {
        match line.parse::<u16>() {
            Ok(num) => num,
            Err(_) => {
                term.write_line(&format!("A valid {} must be provided.", label)).unwrap();
                exit(0);
            }
        }
    } else {
        term.write_line(&format!("A valid {} must be provided.", label)).unwrap();
        exit(0);
    }
}

fn calculate_target_rate(age: u16, resting_rate: u16, intensity: u16) -> u16 {
    (((220 - age) - resting_rate) * intensity / 100) + resting_rate
}
