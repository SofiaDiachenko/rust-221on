use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut color_counts = HashMap::new();

    for &sock in ar {
        *color_counts.entry(sock).or_insert(0) += 1;
    }

    let mut total_pairs = 0;
    for &count in color_counts.values() {
        total_pairs += count / 2;
    }

    total_pairs
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}

