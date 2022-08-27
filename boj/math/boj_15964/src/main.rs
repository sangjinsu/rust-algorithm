use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let result = line.split_whitespace().
        map(|num| num.parse::<isize>().unwrap()).
        map(|num| num * num).
        reduce(|acc, num| {
            acc - num
        }).unwrap();
    println!("{}", result)
}
