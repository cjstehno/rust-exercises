use std::io::{stdin,stdout,Write};

fn main() {
    let mut name = String::new();

    print!("What is your name? ");
    let _= stdout().flush();

    stdin().read_line(&mut name).expect("Did not enter a correct string");
    if let Some('\n') = name.chars().next_back() {
        name.pop();
    }
    if let Some('\r') = name.chars().next_back() {
        name.pop();
    }

    println!("Hello, {}, nice to meet you!", name);
}
