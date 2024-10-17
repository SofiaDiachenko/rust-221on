use std::io::{self, BufRead};

fn breakingRecords(scores: &[i32]) -> Vec<i32> {

    let mut best_count = 0;
    let mut worst_count = 0;

    let mut best_score = scores[0];
    let mut worst_score = scores[0];

    for &score in scores.iter().skip(1) {
        if score > best_score {
            best_score = score;
            best_count += 1;
        } else if score < worst_score {
            worst_score = score;
            worst_count += 1;
        }
    }

    vec![best_count, worst_count]
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = breakingRecords(&scores);

    println!("{} {}", result[0], result[1]);
}
