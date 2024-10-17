#[allow(dead_code)]
enum Direction {
    East,
    West,
    North,
    South,
}
#[test]
fn main1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}
#[test]
fn main2() {
    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0
    };

    assert_eq!(binary, 1);
}
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn main3() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message) {
    match msg {
        Message::Move{x: a, y: b} => { // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants")
    }
}
#[test]
fn main4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }
}

enum MyEnum {
    Foo,
    Bar
}
#[test]
fn main5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e , MyEnum::Foo) { // fix the error with changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);
}
#[test]
fn main6() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }
}
enum Foo {
    Bar(u8),
}
#[test]
fn main7() {
    let a = Foo::Bar(1);

    let Foo::Bar(i) = a;
        println!("foobar holds the value: {}", i);
    }

#[test]
fn main9() {
    let age = Some(30);
    if let Some(age) = age { // create a new variable with the same name as previous `age`
        assert_eq!(age, 30);
    } // the new variable `age` goes out of scope here

    match age {
        // match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}
