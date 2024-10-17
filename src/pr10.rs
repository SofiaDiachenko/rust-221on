fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    // Обчислюємо реальний зсув, враховуючи довжину рядка
    let shift = ((n % len) + len) % len;

    // Якщо shift == 0, рядок залишається незмінним
    if shift == 0 {
        return s;
    }

    // Виконуємо зсув вліво або вправо залежно від значення shift
    let (left, right) = s.split_at((len - shift) as usize);
    right.to_string() + left
}

#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        });
}
