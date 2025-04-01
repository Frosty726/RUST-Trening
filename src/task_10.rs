use std::io::{BufReader, Read};

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Сделайте трейт Read для RotDecoder.
impl <R : Read> Read for RotDecoder <R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Читаем байты, сохраняем их количество в виде опционала
        let bytes_read = self.input.read(buf)?;
        // 26 букв в алфавите
        let range = 26;

        // Проходимся по прочитанным байтам
        for byte in &mut buf[..bytes_read] {
            // Обрабатываем только алфавит
            if byte.is_ascii_alphabetic() {
                // Проверка регистра
                let is_upper = byte.is_ascii_uppercase();
                // Задание смещения на основе регистра
                let offset = if is_upper { b'A' } else { b'a' };
                // Изменяем букву прямо на месте
                *byte = (*byte - offset + self.rot) % range + offset;
            }
        }

        // Возвращаем результат изначального чтения
        Ok(bytes_read)
    }
}

pub fn demonstrate() {
    println!("--- Task 10 ---");
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
    println!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}