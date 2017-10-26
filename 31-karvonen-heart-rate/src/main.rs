extern crate console;
#[macro_use]
extern crate clap;

use clap::{App};
use console::Term;
use std::process::exit;

///
/// Takes input of the age and resting heart rate and generates a table of target heart rates.
///
/// Optional Command Line Arguments:
/// * help (`-h`, `--help`)
/// * age (`-a`, `--age`)
/// * resting heart rate (`-r`, `--resting`)
/// * intensity (`-i`, `--intensity`)
///
/// Note: if the `intensity` is specified, only that value will be returned, otherwise the whole table
/// will be generated.
///
fn main() {
    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let term = Term::stdout();

    let age: u16 = required_input(&term, matches.value_of("age"), "age");
    let resting_heart_rate: u16 = required_input(&term, matches.value_of("resting"), "resting heart rate");

    if let Some(intensity) = optional_input(&term, matches.value_of("intensity"), "intensity"){
        term.write_line(&format!("{} bpm", calculate_target_rate(age, resting_heart_rate, intensity))).unwrap();

    } else {
        term.write_line(&format!("\nResting Pulse: {}  Age: {}\n", resting_heart_rate, age)).unwrap();
        term.write_line("Intensity | Target").unwrap();
        term.write_line("-------------------").unwrap();

        let mut intensity_pct: u16 = 50;
        while intensity_pct < 100 {
            let target = calculate_target_rate(age, resting_heart_rate, intensity_pct);
            term.write_line(&format!("{}%       | {} bpm", intensity_pct, target)).unwrap();

            intensity_pct += 5;
        }
    }
}

/// Parses the input from the command line or prompt and returns a number value.
fn required_input(term: &Term, arg: Option<&str>, label: &str) -> u16 {
    let input = match arg {
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
    };

    match input.parse::<u16>() {
        Ok(num) => num,
        Err(_) => {
            term.write_line(&format!("A valid {} must be provided.", label)).unwrap();
            exit(0);
        }
    }
}

/// Parses the input from the command line for an optional argument.
fn optional_input(term: &Term, arg: Option<&str>, label: &str) -> Option<u16> {
    match arg {
        Some(value) => {
            match value.parse::<u16>() {
                Ok(num) => Some(num),
                Err(_) => {
                    term.write_line(&format!("A valid {} must be provided.", label)).unwrap();
                    exit(0);
                }
            }
        },
        None => None
    }
}

/// Calculates the Karvonen Heart Rate value.
fn calculate_target_rate(age: u16, resting_rate: u16, intensity: u16) -> u16 {
    (((220 - age) - resting_rate) * intensity / 100) + resting_rate
}
