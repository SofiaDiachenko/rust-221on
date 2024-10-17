#[test]
fn main1() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');
    move_ownership(s.clone());
    assert_eq!(s, "hello, world!");
    println!("Success!")
}
fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
#[test]
fn main2() {
    let mut s = String::from("hello, world");
    let slice1: &str = &s; // in two ways
    assert_eq!(slice1, "hello, world");
    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");
    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!")
}
#[test]
fn main4() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1];
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];
    assert_eq!(slice2, "世");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }
    println!("Success!")
}
#[test]
fn main5() {
    let mut s = String::new();
    s.push_str("hello");
    let v = vec![104, 101, 108, 108, 111];
    let s1 = String::from_utf8(v).unwrap();

    assert_eq!(s, s1);

    println!("Success!")
}
#[test]
fn main6() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}
use std::mem;
#[test]
fn main7() {
    let story = String::from("Rust By Practice");
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
    assert_eq!(*story, s);
    println!("Success!")
}

