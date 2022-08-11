fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let x = line.split_whitespace().
        map(|s| s.parse::<isize>().unwrap()).nth(1).unwrap();

    line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .filter(|&num| num < x)
        .for_each(|num| print!("{} ", num));
}
