use std::io::{Read, stdin};

fn main() {
    let mut lines = String::new();
    stdin().read_to_string(&mut lines).unwrap();
    print!("{}", lines)
}
