use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let mut n = String::new();
    reader.read_line(&mut n).unwrap();

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let result: usize = line.trim().split("")
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<usize>().unwrap())
        .sum();
    write!(writer, "{}", result).unwrap()
}
