use rand::Rng;

// Функція для підрахунку мінімальної кількості переносу вантажу
fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    // Перевіряємо, чи можливо розподілити вантаж порівну
    if total % n != 0 {
        return None; // Неможливо розподілити порівну
    }

    let average = total / n;
    let mut moves = 0;
    let mut imbalance = 0;

    for &shipment in shipments.iter() {
        imbalance += shipment as i32 - average as i32; // Прибрано зайві дужки
        moves += imbalance.abs() as usize;
    }

    Some(moves)
}

// Функція для перевірки, чи можливо розподілити вантаж порівну
fn can_distribute_equally(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    total % shipments.len() as u32 == 0
}

// Функція для генерації випадкового вектора, де вантаж може бути розподілений порівну
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![0; n];
    let mut rng = rand::thread_rng();

    let total = (rng.gen_range(10..100) * n as u32) as u32; // Генеруємо загальний вантаж
    let average = total / n as u32;

    // Розподіляємо рівномірно, але з незначними відхиленнями
    for i in 0..n {
        shipments[i] = average + rng.gen_range(0..3); // Додаємо невеликі відхилення
    }

    // Виправляємо, щоб сума була точно кратною n
    let sum: u32 = shipments.iter().sum(); // Прибрано зайву mut
    let diff = (total as i32 - sum as i32) as i32;

    // Виправляємо різницю, щоб досягнути точної суми
    if diff > 0 {
        shipments[0] += diff as u32;
    } else if diff < 0 {
        shipments[0] -= (-diff) as u32;
    }

    shipments
}
#[test]
fn main() {
    let example1 = vec![8, 2, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];

    // Приклад 1
    println!("Shipments: {:?}", example1);
    match count_permutation(&example1) {
        Some(moves) => println!("Answer: {}", moves),
        None => println!("Cannot distribute equally."),
    }

    // Приклад 2
    println!("Shipments: {:?}", example2);
    match count_permutation(&example2) {
        Some(moves) => println!("Answer: {}", moves),
        None => println!("Cannot distribute equally."),
    }

    // Генерація випадкових вантажів, які можна розподілити порівну
    let random_shipments = gen_shipments(5);
    println!("Generated shipments: {:?}", random_shipments);
    println!("Can distribute equally: {}", can_distribute_equally(&random_shipments));
}
