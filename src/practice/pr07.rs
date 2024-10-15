fn print_star(max_width: usize) {
    let spaces = " ".repeat((max_width - 1) / 2);
    println!("{}*", spaces);
}

fn print_triangle(n: usize, max_width: usize) {
    for i in 0..n {
        let spaces = " ".repeat((max_width - (2 * i + 1)) / 2);
        let stars = "* ".repeat(i + 1);
        println!("{}{}", spaces, stars.trim());
    }
}

fn print_christmas_tree(triangle_count: usize) {
    let mut max_width = 0;

    // Обчислення найбільшого трикутника, щоб визначити максимальну ширину ялинки
    for i in 1..=triangle_count {
        let current_width = 2 * i + 1;
        if current_width > max_width {
            max_width = current_width;
        }
    }

    // Друк зірки зверху
    print_star(max_width);

    // Виведення ялинки
    for i in 1..=triangle_count {
        print_triangle(i + 1, max_width);
    }
}
#[test]
fn main() {
    let triangle_count = 6; // Кількість трикутників
    print_christmas_tree(triangle_count);
}