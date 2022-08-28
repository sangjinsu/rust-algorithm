use std::io::stdin;

fn main() {
    let mut max_num = 0;
    let mut max_index = 0;

    for i in 1..10 {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let num = line.trim().parse().unwrap();
        if num > max_num {
            max_num = num;
            max_index = i;
        }
    }
    print!("{}\n{}", max_num, max_index);
}
