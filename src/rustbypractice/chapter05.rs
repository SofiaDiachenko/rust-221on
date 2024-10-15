//5.1
//1
#[test]
fn test1(){
    let x = String::from("Hello world");
    let y = x.clone(); // Clone x
    println!("{}, {}", x, y); // Now you can use both x and y
}
//3
#[test]
fn test2() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert cloned String to Vec
    let _s = s.clone().into_bytes();
    s
}
//4
#[test]
fn test3() {
    let s = String::from("Hello World");
    print_str(&s); // Передаємо посилання на рядок
    println!("{}", s); // Тепер s ще доступна для використання
}
fn print_str(s: &String)  { // Приймаємо посилання на String
    println!("{}", s);
}
//5
#[test]
fn test4() {
    let x = (1, 2, (), "hello"); // &str реалізує Copy, а не String
    let y = x; // Ці типи реалізують Copy, тому тут відбувається копіювання
    println!("{:?}, {:?}", x, y);
}
//6
#[test]
fn test5() {
    let s = String::from("Hello ");
    let mut s1 = s;  // Робимо s1 змінною (mutable)
    s1.push_str("World!");  // Тепер можна змінювати значення
    println!("Success!");
}
//7
#[test]
fn test6() {
    let mut x = Box::new(5);  // Робимо x змінним
    let y = &mut x;  // Змінюване посилання на x
    **y = 4;  // Змінюємо вміст за посиланням
    assert_eq!(*x, 4);  // Перевіряємо, що значення змінилося
    println!("Success!");
}
//8
#[test]
fn test7() {
    let t = (String::from("hello"), String::from("world"));
    // Беремо посилання на перший елемент, щоб не переміщати його
    let _s = &t.0;
    println!("{:?}", t);
}
//9
#[test]
fn test8() {
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = t.clone();
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}
//5.2
//1
#[test]
fn test9() {
    let x = 5;
    let p = &x;
    println!("the memory address of x is {:p}", p);
}
//2
#[test]
fn test10() {
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
}
//3
#[test]
fn test11() {
    let s = String::from("hello, ");
    borrow_object(&s);
}
fn borrow_object(_s: &String) {}
//4
#[test]
fn test12() {
    let mut s = String::from("hello, ");
    push_str(&mut s)
}
fn push_str(s: &mut String) {
    s.push_str("world")
}
//5
#[test]
fn test13() {
    let mut s = String::from("hello, ");
    let p = &mut s;
    p.push_str("world");
}
//6
#[test]
fn test14() {
    let c = '中';
    let r1 = &c;

    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1),get_addr(r2));
}
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
//7
#[test]
fn test15() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
}
//10
#[test]
fn test16() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
}