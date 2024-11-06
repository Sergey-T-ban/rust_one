//cp4.1(10)
use std::ops::{Range, RangeInclusive};
#[test]
fn chp4() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

//cp4.2(6)
use std::mem::size_of_val;
#[test]
fn ch4() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); // Змінюємо 4 на 0

    println!("Success!");
}

//cp4.3(1)
#[test]
fn chs4() {
    let v = {
        let mut x = 1;
        x += 2;
        x // Без крапки з комою, щоб повернути значення x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

//cp4.4(1)
#[test]
fn cp4() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}


