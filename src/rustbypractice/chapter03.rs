#[test]
fn test1() {
    let x: i32 = 5;
    let _y: i32;
    assert_eq!(x, 5);
    println!("Success!");
}
#[test]
fn test2(){
    let mut x = 1; // Додаємо mut для можливості змінювати значення змінної
    x += 2; // Змінюємо значення
    assert_eq!(x, 3); // Перевірка
    println!("Success!");
}
#[test]
fn test3(){
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {}", x);
}
#[test]
fn test4(){
    let x = define_x(); // Викликаємо функцію define_x і присвоюємо її результат змінній x
    println!("{}, world", x);
}
fn define_x() -> &'static str {
    let x = "hello"; // Змінна x
    x // Повертаємо x
}
#[test]
fn test5(){
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // Внутрішня область, тут x = 12
    }
    assert_eq!(x, 5); // Повертаємося до зовнішньої області, тут x = 5
    let x = 42;
    println!("{}", x); // Друкуємо 42
}
#[test]
fn test6() {
    let s: &str = "example";
    let _s1: &str = s; // Префіксуємо змінну, щоб вказати, що вона свідомо не використовується

    println!("Success!");
}

#[test]
fn test7(){
    let x = 1;
    println!("{}", x); // Використовуємо змінну x
}
#[test]
fn test8() {
    let (mut x, y) = (1, 2); // Додаємо mut до x, щоб дозволити зміну
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
#[test]
fn test9() {
    let (x, y);
    (x, ..) = (3, 4);  // x = 3
    [.., y] = [1, 2];  // y = 2
    // Заповнюємо пропуск правильним значенням
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}