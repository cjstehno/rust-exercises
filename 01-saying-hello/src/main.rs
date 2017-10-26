extern crate console;

use console::Term;

const WRITE_ERROR: &'static str = "Unable to write to terminal!";

fn main() {
    let term = Term::stdout();

    term.write_str("What is your name? ").expect(WRITE_ERROR);

    if let Ok(line) = term.read_line() {
        let name: String = if line.len() > 0 { line } else { String::from("World") };
        term.write_line(&format!("Hello, {}, nice to meet you!", name)).expect(WRITE_ERROR);
    }
}

