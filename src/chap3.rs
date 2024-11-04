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

//ch4
#[test]
fn ch4() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}

//ch5
#[test]
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn ch5() {
    let _x: i32 = 5;
    {
        let x = 5;
        assert_eq!(x, 5); // У цьому блоці `x` дорівнює 5
    }

    let x = 12;
    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x); // Prints "42".
}

//ch6
#[test]
fn ch6() {
    let x: i32 = 1;
    // Shadowing and re-binding
    let _x = x;

    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}

//ch7
#[test]
fn ch7_1() {
    let _x = 1;
}

#[test]
fn ch_2() {
    let x = 1;
    println!("{}", x);
}

//ch8
#[test]
fn ch8() {
    let (mut x, y) = (1, 2); // Додаємо `mut` для `x`
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

//ch9
#[test]
fn main() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    // Заповнюємо пропуск
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}
