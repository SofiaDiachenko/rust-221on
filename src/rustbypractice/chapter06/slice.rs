#[test]
fn test() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];

    let _s2: &str = "hello, world";
}
#[test]
fn test2() {
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];
    assert!(std::mem::size_of_val(&slice) == 16);
}
#[test]
fn test3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
}
#[test]
fn test4() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);
}
#[test]
fn test5() {
    let s = "你好，世界";
    let slice = &s[0..3];

    assert!(slice == "你");
}
#[test]
fn test6() {
    let mut s = String::from("hello world");
    let letter = first_letter(&s);
    println!("the first letter is: {}", letter);
    s.clear();
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}