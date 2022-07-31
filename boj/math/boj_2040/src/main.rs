use std;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let numbers =
        line.split_whitespace().
            map(|s| s.parse::<isize>().unwrap()).collect::<Vec<_>>();
    let n = numbers[0];
    let m = numbers[1];
    let result = (n - m).abs();
    println!("{}", result);
}
