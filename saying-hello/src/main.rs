use std::io::{stdin,stdout,Write};

fn main() {
    print!("What is your name? ");
    let _ = stdout().flush();

    let mut buffer = String::new();
    let mut name: String = "World".to_string();

    match stdin().read_line(&mut buffer) {
        Ok(_) => name = if buffer.trim().len() > 0 { buffer.trim().to_string() } else { name },
        Err(_) => panic!("Error reading input.")
    };

    println!("Hello, {}, nice to meet you!", name);
}

