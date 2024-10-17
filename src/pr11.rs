fn is_palindrome(x: u32) -> bool {
    // Якщо число закінчується на 0 і не є 0, то воно не може бути паліндромом
    if x != 0 && x % 10 == 0 {
        return false;
    }

    let mut original = x;
    let mut reversed = 0;

    // Перевертаємо половину числа
    while original > reversed {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }

    // Перевіряємо, чи дорівнює перевернута частина оригіналу або його половині (для чисел з непарною кількістю цифр)
    original == reversed || original == reversed / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
            (10, false),
            (0, true),
        ];

        data.iter().for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
    }
}
