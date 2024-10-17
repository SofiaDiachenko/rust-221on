#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
    hobby: String
}
#[test]
fn main1() {
    let age = 30;
    let _p = Person {
        name: String::from("sunface"),
        age,
        hobby: "coding".to_string()
    };
}
struct Unit;
#[allow(dead_code)]
trait SomeTrait {
    // ...Some behavours defines here
}

// We don't care the the fields are in Unit, but we care its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
#[test]
fn main2() {
    let u = Unit;
    do_something_with_unit(u);
}

// fill the blank to make the code work
fn do_something_with_unit(_u: Unit) {   }
#[allow(dead_code)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[test]
fn main3() {
    let v = Point(0, 127, 255);
    check_color(v);
}

fn check_color(p: Point) {
    let Point(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}
struct Person2 {
    name: String,
    age: u8,
}
#[test]
fn main4() {
    let age = 18;
    let mut p = Person2 {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18?
    p.age = 30;

    p.name = String::from("sunfei");
}
#[allow(dead_code)]
struct Person3 {
    name: String,
    age: u8,
}
#[test]
fn main5() {}
#[allow(dead_code)]
fn build_person(name: String, age: u8) -> Person3 {
    Person3{
        age,
        name
    }
}
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[test]
fn main6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let _u2 = set_email(u1);
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[test]
fn main7() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // print debug info to stderr

    println!("{:?}", rect1); // print debug info to stdout
}
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
#[test]
fn main8() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    println!("{}", f.data);
}