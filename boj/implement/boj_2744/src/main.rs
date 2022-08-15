use std::io::stdin;

fn main() {
    let mut chars = String::new();
    stdin().read_line(&mut chars).unwrap();

    let result = chars.chars().map(|c|
        match c {
            'A'..='Z' => c.to_ascii_lowercase(),
            'a'..='z' => c.to_ascii_uppercase(),
            _ => c
        }).collect::<String>();
    println!("{}", result);
}
