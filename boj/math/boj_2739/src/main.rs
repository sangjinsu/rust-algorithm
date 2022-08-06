fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: isize = line.trim().parse().unwrap();
    let mut result = String::new();
    for i in 1..10 {
        result.push_str(&*format!("{} * {} = {}\n", n, i, n * i))
    }
    print!("{}", result)
}
