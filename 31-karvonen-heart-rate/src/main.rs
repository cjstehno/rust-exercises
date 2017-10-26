extern crate console;
extern crate clap;

use clap::{Arg, App};
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

    let age: u16 = number_input(&term, matches.value_of("age"), "age");

    let resting_heart_rate: u16 = number_input(&term, matches.value_of("resting"), "resting heart rate");

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

fn number_input(term: &Term, arg: Option<&str>, label: &str) -> u16 {
    parse_input(term, resolve_input(term, arg, label), label)
}

fn resolve_input(term: &Term, arg: Option<&str>, label: &str) -> String {
    match arg {
        Some(value) => String::from(value),
        None => {
            term.write_str(&format!("What is your {}? ", label)).unwrap();

            if let Ok(line) = term.read_line() {
                line
            } else {
                term.write_line(&format!("A valid {} must be provided.", label)).unwrap();
                exit(0);
            }
        }
    }
}

fn parse_input(term: &Term, input: String, label: &str) -> u16 {
    match input.parse::<u16>() {
        Ok(num) => num,
        Err(_) => {
            term.write_line(&format!("A valid {} must be provided.", label)).unwrap();
            exit(0);
        }
    }
}

fn calculate_target_rate(age: u16, resting_rate: u16, intensity: u16) -> u16 {
    (((220 - age) - resting_rate) * intensity / 100) + resting_rate
}
