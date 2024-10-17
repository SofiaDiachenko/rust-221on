//6.2
#[test]
fn test1() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    assert!(arr.len() == 5);
}
#[test]
fn test2() {
    let _arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    assert!(std::mem::size_of_val(&arr) == 12);
}
#[test]
fn test3() {
        let list: [i32; 100] = [1; 100];

        assert!(list[0] == 1);
        assert!(list.len() == 100);
}
#[test]
fn test4() {
    // fix the error
    let _arr = [1, 2, 3];
}
#[test]
fn test5() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0];

    assert!(ele == 'a');
}
#[test]
fn test6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    let _name0 = names.get(0).unwrap();
    let _name1 = &names[1];
}