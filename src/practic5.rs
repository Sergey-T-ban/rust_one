const WIDTH: usize = 28;  // Ширина конверта
const HEIGHT: usize = 14; // Висота конверта
#[test]
fn main() {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 || j == 0 || j == WIDTH - 1 || j == i * 2 || j == WIDTH - i * 2 - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}







