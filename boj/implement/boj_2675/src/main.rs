use std::io::stdin;

fn main() {
    let mut test_cases = String::new();
    stdin().read_line(&mut test_cases).unwrap();
    let test_cases = test_cases.trim().parse().unwrap();
    for _ in 0..test_cases {
        let mut rs = String::new();
        stdin().read_line(&mut rs).unwrap();
        let mut rs = rs.trim().split_whitespace();
        let r = rs.next().unwrap().parse::<usize>().unwrap();
        let s = rs.next().unwrap();
        let result = s
            .chars()
            .map(|ch| ch.to_string().repeat(r))
            .reduce(|mut acc, item| acc + &item)
            .unwrap();
        println!("{}", result)
    }
}
