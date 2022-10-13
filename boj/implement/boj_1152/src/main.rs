use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let result = line.split_whitespace().count();

    write!(writer, "{}", result).unwrap();
    line.clear();
}
