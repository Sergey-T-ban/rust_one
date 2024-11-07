fn find_variables() -> (char, char, char, char, char, char, char, char) {
    // Приклад значень для змінних
    let m = 'm';
    let u = 'u';
    let x = 'x';
    let a = 'a';
    let s = 's';
    let l = 'l';
    let o = 'o';
    let n = 'n';

    (m, u, x, a, s, l, o, n)
}

fn print_result() {
    let (m, u, x, a, s, l, o, n) = find_variables();

    // Виправлене форматування: кожен аргумент має своє місце у форматі
    println!("{}{}{}", m, u, x);  // Вивести 'mux'
    println!("{}", a);  // Вивести 'a'
    println!("{}        {}", x, a);  // Вивести 'x        a'
    println!("  ------");             // Вивести '------'
    println!("    {}{}{}", s, l, o);  // Вивести 'slo'
    println!("{}", n);  // Вивести 'n'
}
#[test]
fn main() {
    print_result();
}
