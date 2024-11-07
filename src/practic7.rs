fn draw_tree(triangles: u32) {
    let max_width = 2 * (triangles + triangles - 1) + 1; // Максимальна ширина для вирівнювання

    // Малюємо кожен трикутник
    for i in 0..triangles {
        let _width = 2 * i + 2; // Ширина трикутника
        let height = i + 1;    // Висота трикутника

        for j in 0..height {
            let spaces = (max_width - (2 * j + 1)) / 2; // Центруємо трикутник
            print!("{:width$}", "", width = spaces as usize); // Виводимо пробіли
            for _ in 0..(2 * j + 1) {
                print!("*"); // Виводимо зірочки
            }
            println!(); // Перехід на новий рядок
        }
    }

    // Малюємо стовп ялинки
    let trunk_width = 3;
    let trunk_height = 0;

    for _ in 0..trunk_height {
        let spaces = (max_width - trunk_width) / 2; // Центруємо стовп
        println!("{:width$}***", "", width = spaces as usize); // Виводимо стовп
    }
}
#[test]
fn main() {
    let triangles = 6; // Кількість трикутників
    draw_tree(triangles);
}


