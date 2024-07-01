use std::io::{self, Write};

fn read_console(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    input.to_string()
}

pub fn input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let input = read_console(prompt);

        if let Ok(shit) = input.parse::<T>() {
            return shit;
        } else {
            println!("Enter a {}", std::any::type_name::<T>());
        }
    }
}

pub fn input_constrained<T: std::str::FromStr>(prompt: &str, p: fn(&T) -> bool) -> T {
    loop {
        let input: T = input(prompt);

        if p(&input) {
            return input;
        } else {
            println!("Did not meet the constraints");
        }
    }
}

pub fn input_words(prompt: &str) -> Vec<String> {
    let input: String = input(prompt);
    let words = input
        .split_whitespace()
        .map(|str| str.to_string())
        .collect::<Vec<_>>();
    words
}

pub fn input_words_exact(prompt: &str, n: usize) -> Vec<String> {
    loop {
        let words = input_words(prompt);

        if words.len() == n {
            return words;
        } else {
            println!("Please input exactly {n} words");
        }
    }
}
