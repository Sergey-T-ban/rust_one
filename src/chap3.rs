// ch1
#[test]
fn ch1() {
    let x: i32 = 5; // Ініціалізуємо x значенням 5
    let _y: i32; // Залишаємо y неініціалізованою, попередження

    assert_eq!(x, 5);
    println!("Success!");
}

// ch2
#[test]
fn ch2() {
    let mut x = 1; // Позначаємо x як змінну
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

//ch3
#[test]
fn ch3() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {}", x); // Видалено посилання на y
}
