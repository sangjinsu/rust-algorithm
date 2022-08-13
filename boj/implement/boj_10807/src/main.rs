use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let nums = input.split_whitespace().map(|num| num.parse::<isize>().unwrap()).collect::<Vec<_>>();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let x = input.trim().parse::<isize>().unwrap();
    let result = nums.iter().filter(|&num| *num == x).count();
    print!("{}", result)
}
