extern crate console;

use console::Term;

// TODO: make version that takes cli args or presents with questions
// TODO: error handling

fn main() {
    let term = Term::stdout();

    term.write_str("How old are you? ").unwrap();

    let mut age: u16 = 0;

    if let Ok(line) = term.read_line() {
        if let Ok(num) = line.parse::<u16>() {
            age = num;
        } else {
            panic!("The specified age is invalid!");
        }
    }

    term.write_str("What is your resting heart rate? ").unwrap();

    let mut resting_heart_rate: u16 = 0;

    if let Ok(line) = term.read_line() {
        if let Ok(num) = line.parse::<u16>() {
            resting_heart_rate = num;
        } else {
            panic!("The specified heart rate is invalid!");
        }
    }

    term.write_line(&format!("\nResting Pulse: {}  Age: {}\n", resting_heart_rate, age)).unwrap();

    term.write_line("Intensity | Target").unwrap();
    term.write_line("-------------------").unwrap();

    for intensity in 55..96 {
        let target = calculate_target_rate(age, resting_heart_rate, intensity);
        term.write_line(&format!("{}%       | {} bpm", intensity, target)).unwrap();
    }
}

fn calculate_target_rate(age: u16, resting_rate: u16, intensity: u16) -> u16 {
    (((220 - age) - resting_rate) * intensity / 100) + resting_rate
}
