struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
#[test]
fn main1() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);
}
#[derive(Debug)]
struct TrafficLight {
    color: String,
}
#[allow(dead_code)]
impl TrafficLight {
    pub fn show_state_owned(&self) {
        println!("the current state is {}", self.color);
    }
}
#[test]
fn main2() {
    let light = TrafficLight {
        color: "red".to_owned(),
    };
    light.show_state();
    println!("{:?}", light);
}
#[allow(dead_code)]
struct TrafficLight2 {
    color: String,
}
#[allow(dead_code)]
impl TrafficLight {
    pub fn show_state(self: &Self) {
        println!("the current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
#[allow(dead_code)]
struct Rectangle1 {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle1 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
#[allow(dead_code)]
impl Rectangle1 {
    fn can_hold(&self, other: &Rectangle1) -> bool {
        self.width > other.width && self.height > other.height
    }
}
#[test]
fn main5() {}
#[allow(dead_code)]
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    fn color(&self) -> String {
        match *self {
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Yellow => "yellow".to_string(),
            TrafficLightColor::Green => "green".to_string(),
        }
    }
}
#[test]
fn main6() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}