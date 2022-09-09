use std::collections::HashMap;
use std::io::{BufWriter, stdin, stdout};
use std::io::Write;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let input = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let (m, n) = (input[0], input[1]);
    let mut monsters: HashMap<String, u32> = HashMap::new();
    let mut names: HashMap<u32, String> = HashMap::new();

    for i in 1..m + 1 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let value = input.trim().to_string();
        monsters.insert(value.clone(), i);
        names.insert(i, value.clone());
    }
    let mut writer = BufWriter::new(stdout());
    for _ in 0..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let value = input.trim().to_string();
        match value.parse::<u32>() {
            Ok(number) => writeln!(writer, "{}", names.get(&number).unwrap()).unwrap(),
            Err(_) => writeln!(writer, "{}", monsters.get(&value).unwrap()).unwrap()
        };
    }
}
