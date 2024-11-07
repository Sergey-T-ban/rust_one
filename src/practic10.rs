fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s; // Якщо рядок порожній, повертаємо його без змін.
    }

    // Приводимо n до діапазону від 0 до len-1
    let n = ((n % len as isize) + len as isize) % len as isize;

    // Здійснюємо циклічний зсув
    let n = n as usize;
    let rotated = format!("{}{}", &s[len - n..], &s[..len - n]);

    rotated
}

#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(
            rotate(s.clone(), *n),
            exp.to_string()
        );
    });
}


