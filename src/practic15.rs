fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];  // Повертаємо порожній рядок для n = 0
    }

    let mut result = vec!["0".to_string(), "1".to_string()];

    for _i in 1..n {
        let len = result.len();
        let mut new_result = Vec::with_capacity(len * 2);

        // Додаємо 0 до початку кожного елемента першої половини
        for code in &result {
            new_result.push(format!("0{}", code));
        }

        // Додаємо 1 до початку кожного елемента другої половини (без зворотного порядку)
        for code in result {
            new_result.push(format!("1{}", code));
        }

        result = new_result;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec!("")),
            (1, vec!("0", "1")),
            (2, vec!("00", "01", "10", "11")),
            (3, vec!("000", "001", "010", "011", "100", "101", "110", "111")),
            (4, vec!("0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111",
                     "1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111")),
        ];

        test_data.iter().for_each(|(n, out)| {
            assert_eq!(gray(*n), *out);
        });
    }
}









