#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}
#[allow(dead_code)]
enum Number1 {
    Zero = 0,
    One,
    Two,
}
#[allow(dead_code)]
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}
#[test]
fn main1() {
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
}
#[allow(dead_code)]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn main2() {
    let _msg1 = Message2::Move{x: 1, y: 2}; // instantiating with x = 1, y = 2
    let _msg2 = Message2::Write(String::from("hello, world")); // instantiating with "hello, world!"
}
#[allow(dead_code)]
enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn main3() {
    let msg = Message1::Move{x: 1, y: 1};

    if let Message1::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }
}
#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn main4() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}
#[test]
fn main5() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);
        return
    }

    panic!("NEVER LET THIS RUN！");
}
#[allow(dead_code)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
