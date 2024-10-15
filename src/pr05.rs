const SIZE: usize = 21; // Розмір конверта (повинен бути непарним для правильного перетину діагоналей)
#[test]
fn main() {
    let mut envelope = String::new();

    for y in 0..SIZE {
        for x in 0..SIZE {
            if y == 0 || y == SIZE - 1 {
                // Верхня і нижня границі
                envelope.push('*');
            } else if x == 0 || x == SIZE - 1 {
                // Ліва і права границі
                envelope.push('*');
            } else if x == y || x == SIZE - y - 1 {
                // Діагональні лінії, що перетинаються посередині
                envelope.push('*');
            } else {
                // Порожні місця всередині
                envelope.push(' ');
            }
        }
        envelope.push('\n');
    }

    print!("{}", envelope);
}
