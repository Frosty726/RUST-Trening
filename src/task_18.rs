//Оригинальная функция
// pub fn luhn(cc_number: &str) -> bool {
//     let mut sum = 0;
//     let mut double = false;

//     for c in cc_number.chars().rev() {
//         if let Some(digit) = c.to_digit(10) {
//             if double {
//                 let double_digit = digit * 2;
//                 sum +=
//                     if double_digit > 9 { double_digit - 9 } else { double_digit };
//             } else {
//                 sum += digit;
//             }
//             double = !double;
//         } else {
//             continue;
//         }
//     }

//     sum % 10 == 0
// }

pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;

    // Единственное исправлени - ограничение на длину чисел
    if cc_number.len() < 2 {
        return false
    }

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            continue;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        // Валидные номера
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        // Невалидные номера
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_invalid_length() {
        // Должно быть больше 2 цифр
        assert!(!luhn(""));
        assert!(!luhn("7"));
        assert!(luhn("42"));
    }

    #[test]
    fn test_with_non_digit_characters() {
        // Тесты с символами, не являющимися цифрами
        assert!(luhn("4539-3195-0343-6467"));
        assert!(!luhn("8273#1232!7352@0569"));
    }
}

pub fn demonstrate() {
    println!("--- Task 18 ---");
    println!("Запустите cargo test.");
    println!();
}