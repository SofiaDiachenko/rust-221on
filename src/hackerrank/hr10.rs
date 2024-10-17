use std::io::{self, BufRead};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut rounded_grades = Vec::new();

    for &grade in grades {
        if grade < 38 {
            rounded_grades.push(grade);
        } else {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;

            if next_multiple_of_5 - grade < 3 {
                rounded_grades.push(next_multiple_of_5);
            } else {
                rounded_grades.push(grade);
            }
        }
    }

    rounded_grades
}
#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);
    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = grading_students(&grades);

    for grade in result {
        println!("{}", grade);
    }
}
