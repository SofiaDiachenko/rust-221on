use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'simpleArraySum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY ar as parameter.
 */

#[allow(dead_code)]
fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Open file for writing
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the count of elements in the array (this value is not used)
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the array and convert it into a vector of integers
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Call the function and write the result to the file
    let result = simple_array_sum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}
