trait Bird {
    fn quack(&self) -> String;
}
struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
#[allow(dead_code)]
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}
impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}
impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}
#[test]
fn main1() {
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");
    println!("Success!")
}

fn hatch_a_bird(species: u8) ->Box<dyn Bird> {
    if species == 1 {
        Box::new(Swan{})
    } else {
        Box::new(Duck{})
    }
}
trait Bird1 {
    fn quack(&self);
}

struct Duck1;
#[allow(dead_code)]
impl Duck1 {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan1;
#[allow(dead_code)]
impl Swan1 {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}
impl Bird1 for Duck1 {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}
impl Bird1 for Swan1 {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}
#[test]
fn main2() {
    let birds: [Box<dyn Bird1>; 2] = [Box::new(Duck1 {}), Box::new(Swan1 {})];
    for bird in birds {
        bird.quack();
    }
}
trait Draw {
    fn draw(&self) -> String;
}
impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}
#[test]
fn main3() {
    let x = 1.1f64;
    let y = 8u8;
    draw_with_box(Box::new(x));
    draw_with_ref(&y);
}
fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}
fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}
trait Foo {
    fn method(&self) -> String;
}
impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}
impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}
fn static_dispatch<T: Foo>(x: T) {
    x.method();
}
fn dynamic_dispatch(x: &dyn Foo) {
    x.method();
}
#[test]
fn main4() {
    let x = 5u8;
    let y = "Hello".to_string();
    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!")
}
trait MyTrait {
    fn f(&self) -> Self;
}
impl MyTrait for u32 {
    fn f(&self) -> u32 { 42 }
}
impl MyTrait for String {
    fn f(&self) -> String { self.clone() }
}
fn my_function(x: impl MyTrait) -> impl MyTrait  {
    x.f()
}
#[test]
fn main5() {
    my_function(13_u32);
    my_function(String::from("abc"));
}