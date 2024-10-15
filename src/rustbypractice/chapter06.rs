//6.1
#[test]
fn test1() {
    let _s: &str = "hello, world";

    println!("Success!");
}
#[test]
fn test2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s) // Передаємо посилання на `Box<str>`, яке автоматично конвертується в `&str`
}

fn greetings(s: &str) {
    println!("{}", s)
}
#[test]
fn test3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}
#[test]
fn test4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}
#[test]
fn test5() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}
#[test]
fn test6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}