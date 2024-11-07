const SIZE: usize = 5; // Вкажіть тут розмір ромба
#[test]
fn main() {
    for i in 0..SIZE * 2 - 1 {
        let stars = if i < SIZE { i * 2 + 1 } else { (SIZE * 2 - i - 2) * 2 + 1 };
        let spaces = (SIZE * 2 - 1 - stars) / 2;

        print!("{}", " ".repeat(spaces)); // Друкуємо пробіли
        println!("{}", "*".repeat(stars)); // Друкуємо зірочки
    }
}