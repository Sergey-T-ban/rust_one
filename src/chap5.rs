//ch5.1()
#[test]
fn cp5() {
    let x = Box::new(5);

    let mut y = x;  // оновлено цей рядок

    *y = 4;

    assert_eq!(*y, 4);  // також перевіримо y

    println!("Success!");
}

//ch5.2(6)
#[test]
fn cp5_1() {
    let c = '中';

    let r1 = &c;
    // Fill the blank, don't change other code
    let ref r2 = c; // Використовуємо `ref` для створення посилання

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}