use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {

        if x1 == x2 {
            "YES".to_string()
        } else {
            "NO".to_string()
        }
    } else {

        if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) > 0 {
            "YES".to_string()
        } else {
            "NO".to_string()
        }
    }
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    let x1 = first_multiple_input[0];
    let v1 = first_multiple_input[1];
    let x2 = first_multiple_input[2];
    let v2 = first_multiple_input[3];

    let result = kangaroo(x1, v1, x2, v2);

    writeln!(&mut fptr, "{}", result).ok();
}
