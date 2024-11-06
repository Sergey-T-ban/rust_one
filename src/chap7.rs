//ch7 (6)
#[test]
fn cp7() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n <= 10 { // Перевірка умови циклу
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1; // Збільшуємо n для наступної ітерації
    }

    println!("n reached {}, so loop is over", n);
}
