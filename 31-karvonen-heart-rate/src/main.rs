extern crate console;

use console::Term;
use std::process::exit;

// TODO: make version that takes cli args or presents with questions

fn main() {
    let term = Term::stdout();

    term.write_str("How old are you? ").unwrap();

    let age: u16 = parse_number_input(&term, "age");

    term.write_str("What is your resting heart rate? ").unwrap();

    let resting_heart_rate: u16 = parse_number_input(&term, "resting heart rate");

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
