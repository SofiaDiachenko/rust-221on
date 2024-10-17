fn sum(x: i32, y: i32) -> i32 {
    x + y
}
#[test]
fn test1(){
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

#[test]
fn test2(){
    print();
}

fn print() -> () {
    println!("Success!");
}

#[test]
fn test5(){
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
