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

    // Отримати значення змінної середовища OUTPUT_PATH, якщо вона існує
    let output_path = match env::var("OUTPUT_PATH") {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Error: OUTPUT_PATH environment variable is not set");
            return;
        }
    };

    // Відкрити файл для запису
    let mut fptr = match File::create(output_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: Could not create file: {}", e);
            return;
        }
    };

    // Прочитати кількість елементів у масиві (ця змінна не використовується)
    let _ar_count = match stdin_iterator.next() {
        Some(Ok(line)) => match line.trim().parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Error: Invalid input for array count");
                return;
            }
        },
        _ => {
            eprintln!("Error: Could not read array count");
            return;
        }
    };

    // Прочитати масив і конвертувати його у вектор цілих чисел
    let ar: Vec<i32> = match stdin_iterator.next() {
        Some(Ok(line)) => line
            .trim_end()
            .split(' ')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect(),
        _ => {
            eprintln!("Error: Could not read array values");
            return;
        }
    };

    // Викликати функцію і записати результат у файл
    let result = simple_array_sum(&ar);

    if let Err(e) = writeln!(&mut fptr, "{}", result) {
        eprintln!("Error: Could not write to file: {}", e);
    }
}
