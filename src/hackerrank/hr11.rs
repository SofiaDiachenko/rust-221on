use std::io::{self, BufRead};

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apples_on_house = apples.iter()
        .map(|&distance| a + distance)
        .filter(|&position| position >= s && position <= t)
        .count();

    let oranges_on_house = oranges.iter()
        .map(|&distance| b + distance)
        .filter(|&position| position >= s && position <= t)
        .count();

    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let s = first_multiple_input[0];
    let t = first_multiple_input[1];

    let second_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let a = second_multiple_input[0];
    let b = second_multiple_input[1];

    let third_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let _m = third_multiple_input[0];
    let _n = third_multiple_input[1];

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}
