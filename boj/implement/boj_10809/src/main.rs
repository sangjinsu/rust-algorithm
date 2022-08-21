use std::io::stdin;

fn main() {
    let mut word = String::new();
    stdin().read_line(&mut word).unwrap();

    let mut result = [-1isize; ("z".as_bytes()[0] - "a".as_bytes()[0] + 1) as usize];
    let word_bytes = word.trim().as_bytes();
    for i in 0..word_bytes.len() {
        let i1 = (word_bytes[i] - "a".as_bytes()[0]) as usize;
        if result[i1] == -1 {
            result[i1] = i as isize
        }
    }
    println!("{}", result.map(|v| v.to_string()).join(" "));
}
