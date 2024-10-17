#[test]
fn main1() {
    let arr: [u8; 3] = [1, 2, 3];
    let v = Vec::from(arr);
    is_vec(&v);
    let v = vec![1, 2, 3];
    is_vec(&v);
    let v = vec!(1, 2, 3);
    is_vec(&v);
    let mut v1 = Vec::new();
    for i in &v {
        v1.push(*i)
    }
    is_vec(&v1);
    assert_eq!(format!("{:?}",v), format!("{:?}",v1));
    println!("Success!")
}
fn is_vec(_v: &Vec<u8>) {}
#[test]
fn main2() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    let mut v2 = Vec::new();
    v2.extend([1, 2, 3]);
    assert_eq!(format!("{:?}",v1), format!("{:?}",v2));

    println!("Success!")
}
#[test]
fn main3() {
    // array -> Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    println!("Success!")
}
#[test]
fn main4() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i))
    }

    for i in 0..5 {
        if let Some(x) = v.get(i) {
            v[i] = x + 1
        } else {
            v.push(i + 2)
        }
    }

    assert_eq!(format!("{:?}",v), format!("{:?}", vec![2, 3, 4, 5, 6]));

    println!("Success!")
}
#[test]
fn main5() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    let slice2 = &v[0..3];

    assert_eq!(slice1, slice2);
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..4];
    slice3[3] = 42;

    assert_eq!(slice3, &[1, 2, 3, 42]);
    assert_eq!(v, &[1, 2, 3, 42]);

    println!("Success!");
}
#[test]
fn main6() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);
    println!("Success!")
}
#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}
#[test]
fn main7() {
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}
trait IpAddr1 {
    fn display(&self);
}

struct V4(String);
impl IpAddr1 for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr1 for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}
#[test]
fn main8() {
    let v: Vec<Box<dyn IpAddr1>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}