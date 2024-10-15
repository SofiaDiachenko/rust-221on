const SIZE: usize = 5;
#[test]
fn main() {
    let mut diamond = String::new();

    // Верхня частина ромба (включаючи середній ряд)
    for i in 0..SIZE {
        let spaces = SIZE - i - 1;
        let stars = 2 * i + 1;
        diamond.push_str(&" ".repeat(spaces));
        diamond.push_str(&"*".repeat(stars));
        diamond.push('\n');
    }

    // Нижня частина ромба
    for i in (0..SIZE - 1).rev() {
        let spaces = SIZE - i - 1;
        let stars = 2 * i + 1;
        diamond.push_str(&" ".repeat(spaces));
        diamond.push_str(&"*".repeat(stars));
        diamond.push('\n');
    }

    print!("{}", diamond);
}