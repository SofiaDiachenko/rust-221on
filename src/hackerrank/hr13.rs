use std::io::{self, BufRead};

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: i32, y: i32) -> i32 {
    (x * y) / gcd(x, y)
}

fn lcm_array(arr: &[i32]) -> i32 {
    arr.iter().fold(1, |acc, &num| lcm(acc, num))
}

fn gcd_array(arr: &[i32]) -> i32 {
    arr.iter().fold(arr[0], |acc, &num| gcd(acc, num))
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = lcm_array(a);
    let gcd_b = gcd_array(b);

    let mut count = 0;
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }
    count
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _first_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    println!("{}", total);
}
