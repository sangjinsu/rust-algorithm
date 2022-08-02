use std::fmt::Write;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse().unwrap();
    let mut result = String::new();
    for i in 1..=n {
        writeln!(&mut result, "{}", i).unwrap();
    }
    print!("{}", result);
}
