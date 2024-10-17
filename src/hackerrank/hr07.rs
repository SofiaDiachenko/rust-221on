use std::io::{self, BufRead};

fn mini_max_sum(arr: &[i32]) {

    let arr: Vec<i64> = arr.iter().map(|&x| x as i64).collect();
    let total_sum: i64 = arr.iter().sum();

    let min_value = arr.iter().min().unwrap();
    let max_value = arr.iter().max().unwrap();

    let min_sum = total_sum - max_value;
    let max_sum = total_sum - min_value;

    println!("{} {}", min_sum, max_sum);
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    mini_max_sum(&arr);
}