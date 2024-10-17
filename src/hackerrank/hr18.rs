use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'dayOfProgrammer' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER year as parameter.
 */

fn dayOfProgrammer(year: i32) -> String {
    if year == 1918 {
        return format!("26.09.{}", year);
    }

    let is_leap = if year < 1918 {
        year % 4 == 0
    } else {
        (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
    };

    if is_leap {
        format!("12.09.{}", year)
    } else {
        format!("13.09.{}", year)
    }
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = dayOfProgrammer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
