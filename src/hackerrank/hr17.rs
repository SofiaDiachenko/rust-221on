use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut frequencies = [0; 6]; // We use index 1-5 for bird types 1 to 5

    for &bird in arr {
        frequencies[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for type_id in 1..=5 {
        if frequencies[type_id] > max_count || (frequencies[type_id] == max_count && result > type_id as i32) {
            max_count = frequencies[type_id];
            result = type_id as i32;
        }
    }

    result
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    println!("{}", result);
}
