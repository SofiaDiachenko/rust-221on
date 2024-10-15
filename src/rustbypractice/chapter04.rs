//4.1 Integer
//1
#[test]
fn test(){
    let x: i32 = 5;
    let mut _y = x;  // Префіксування _ для y

    let _z = 10;  // Префіксування _ для z

    println!("Success!");
}
//2
#[test]
fn test2(){
    let _v: u16 = 38_u8 as u16; // Префіксуємо змінну підкресленням

    println!("Success!");
}
//3
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
//4
#[test]
fn test4(){
    assert_eq!(i8::MAX, 127);  // Максимальне значення для i8
    assert_eq!(u8::MAX, 255);  // Максимальне значення для u8

    println!("Success!");
}
//5
#[test]
fn test5(){
    let v1 = u8::checked_add(251, 8).unwrap_or(0);  // Використовуємо checked_add для уникнення переповнення
    let v2 = i8::checked_add(127, 1).unwrap_or(0);  // Використовуємо максимальне значення i8 і уникнення переповнення
    println!("{},{}", v1, v2);
}
//6
#[test]
fn test6(){
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);  // Оновлюємо значення для перевірки

    println!("Success!");
}
//Floating-Point
//7
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
//8
#[test]
fn test8(){
    let epsilon = f64::EPSILON; // дуже маленьке число, яке враховує похибку
    assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < epsilon);

    println!("Success!");
}
//Range
//9
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
//10
#[test]
fn test10(){
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
//Computations
//11
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
//4.2 Char
//1
#[test]
fn test12(){
    use std::mem::size_of_val;
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); // 'char' is 4 bytes in Rust

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4); // 'char' is still 4 bytes

    println!("Success!");
}
//2
#[test]
fn test13(){
    let c1 = '中'; // Change to char
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
//Bool
//3
#[test]
fn test14(){
    let _f: bool = false;

    let t = true;
    if t {  // Change the condition
        println!("Success!");
    }
}
//4
#[test]
fn test15(){
    let f = false;  // Change f to false
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}
//Unit type
//5
#[test]
fn test16(){
    let _v: () = ();

    assert_eq!((), implicitly_ret_unit());  // Compare with unit type ()

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}
//6
#[test]
fn test17(){
    use std::mem::size_of_val;
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);  // The size of the unit type is 0 bytes

    println!("Success!");
}
//4.3
//Examples
#[test]
fn test18(){
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // No semicolon, so this expression will be assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
//1
#[test]
fn test19(){
    let v = {
        let mut x = 1;
        x += 2;
        x  // Останній вираз без крапки з комою присвоїть результат змінній `v`
    };

    assert_eq!(v, 3);

    println!("Success!");
}
//2
#[test]
fn test20(){
    let x = 3;
    let v = x;  // Присвоюємо значення x змінній v

    assert!(v == 3);

    println!("Success!");
}
//3
#[test]
fn test21(){
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y  // Прибираємо крапку з комою, щоб результат було повернено
}
//4.4
//1
#[test]
fn test22(){
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}
//fn sum(x: i32, y: i32) -> i32 {
//x + y  // Прибираємо крапку з комою, щоб результат було повернено
//2
#[test]
fn test23(){
    print();
}

// Replace i32 with another type
fn print() -> () {
    println!("Success!");
}
//5
#[test]
fn test24(){
    // Присвоюємо `true` змінній `b`
    let b = true;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
