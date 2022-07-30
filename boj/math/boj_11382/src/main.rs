use std;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let answer: u64 =
        line.split_whitespace().
        map(|s| s.parse::<u64>().unwrap()).sum();
    print!("{}", answer)
}
