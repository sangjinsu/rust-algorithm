use std::collections::HashSet;
use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};

fn main() {
    let stdout = stdout();
    let stdin = stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut stdout = BufWriter::new(stdout.lock());

    let mut m = String::new();
    stdin.read_line(&mut m).unwrap();
    let m = m.trim().parse().unwrap();

    let mut num_set: HashSet<i32> = HashSet::new();
    for _ in 0..m {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let (command, num) = {
            let mut inputs = line.split_whitespace();

            (inputs.next().unwrap(), {
                if let Some(num) = inputs.next() {
                    Some(num.parse::<i32>().unwrap())
                } else {
                    None
                }
            })
        };

        match command {
            "add" =>
                { num_set.insert(num.unwrap()); }

            "remove" =>
                { num_set.remove(&num.unwrap()); }

            "toggle" => {
                match num_set.contains(&num.unwrap()) {
                    false =>
                        { num_set.insert(num.unwrap()); }
                    true =>
                        { num_set.remove(&num.unwrap()); }
                }
            }
            "all" => (1..=20).for_each(|i| {
                num_set.insert(i);
            }),
            "empty" => { num_set.clear() }
            "check" => { writeln!(stdout, "{}", num_set.contains(&num.unwrap()) as i32).unwrap() }
            _ => unreachable!(),
        }

        line.clear();
    }
}
