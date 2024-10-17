#[test]
fn test(){
    let x: i32 = 5;
    let mut _y = x;  // Префіксування _ для y

    let _z = 10;  // Префіксування _ для z

    println!("Success!");
}

#[test]
fn test2(){
    let _v: u16 = 38_u8 as u16; // Префіксуємо змінну підкресленням

    println!("Success!");
}

#[test]
fn test3(){
    let x = 5;
    // Виправляємо тип з "u32" на "i32", оскільки компілятор визначає x як i32
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}
// Функція для отримання типу змінної у вигляді рядка
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test4(){
    assert_eq!(i8::MAX, 127);  // Максимальне значення для i8
    assert_eq!(u8::MAX, 255);  // Максимальне значення для u8

    println!("Success!");
}

#[test]
fn test5(){
    let v1 = u8::checked_add(251, 8).unwrap_or(0);  // Використовуємо checked_add для уникнення переповнення
    let v2 = i8::checked_add(127, 1).unwrap_or(0);  // Використовуємо максимальне значення i8 і уникнення переповнення
    println!("{},{}", v1, v2);
}

#[test]
fn test6(){
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);  // Оновлюємо значення для перевірки

    println!("Success!");
}

#[test]
fn test7(){
    let x = 1_000.000_1; // тип за замовчуванням — f64
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of1(&x), "f64".to_string()); // Перевіряємо, що тип x — f64

    println!("Success!");
}

fn type_of1<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
#[test]
fn test8(){
    let epsilon = f64::EPSILON; // дуже маленьке число, яке враховує похибку
    assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < epsilon);

    println!("Success!");
}
#[test]
fn test9(){
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5); // Змінюємо перевірку на правильну суму

    for c in 'a'..='z' {
        println!("{}", c as u8); // Виводимо ASCII-коди замість символів
    }
}
#[test]
fn test10(){
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
#[test]
fn test11(){
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);

    // Fix the type of 1u8 to 1i8 to allow negative numbers
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    // Use integer division to avoid a floating-point mismatch
    assert!(9.6f32 / 3.2f32 == 3.0);

    assert!(24 % 5 == 4);

    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}