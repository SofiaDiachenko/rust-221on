#[test]
fn test1(){
    use std::mem::size_of_val;
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}

#[test]
fn test2(){
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}

#[test]
fn test3(){
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

#[test]
fn test4(){
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

#[test]
fn test5(){
    let _v: () = ();

    assert_eq!((), implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

#[test]
fn test6(){
    use std::mem::size_of_val;
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}