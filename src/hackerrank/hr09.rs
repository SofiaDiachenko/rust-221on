use std::io::{self, BufRead};

fn time_conversion(s: &str) -> String {
    let am_pm = &s[s.len() - 2..];

    let mut hour: i32 = s[0..2].parse().unwrap();

    if am_pm == "PM" && hour != 12 {
        hour += 12;
    }

    if am_pm == "AM" && hour == 12 {
        hour = 0;
    }

    format!("{:02}:{}", hour, &s[3..8])
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);
    println!("{}", result);
}
