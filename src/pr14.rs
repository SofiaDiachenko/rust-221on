use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Rectangle {
    a: Point, // Ліва верхня точка
    b: Point, // Права нижня точка
}

// Функція, що рахує зайняту площу
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied_points = HashSet::new();

    for rect in xs {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                occupied_points.insert((x, y));
            }
        }
    }

    occupied_points.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}
#[test]
fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Total occupied area: {}", occupied);
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
