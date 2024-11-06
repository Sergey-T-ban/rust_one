//ch6.1(5)
#[test]
fn cp6_1() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

//ch6.2(6)
#[test]
fn cp6_2() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // Use get to safely access the array
    let name1 = names.get(2);

    // Optionally handle the case when the index is out of bounds
    match name1 {
        Some(name) => println!("{}", name),
        None => println!("Index out of bounds"),
    }

    println!("Success!");
}

//ch6.3(4)
#[test]
fn cp6_3() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Use a range expression based on `slice1`
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

//ch6.4(6)
#[test]
fn cp6_4() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2, 3));  // Pass a tuple (2, 3)

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

//ch6.5(8)

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
#[test]
fn cp6_5() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let name = &f.name;  // Borrow name as reference
    let data = &f.data;  // Borrow data as reference

    // Now we can print name, data and the full struct f
    println!("{}, {}, {:?}", name, data, f);
}

//ch6.6(3)
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn cp6_6() {
    let msg = Message::Move { x: 1, y: 2 };

    if let Message::Move { x, y } = msg { // Деструктуризація значень x та y
        assert_eq!(x, 1);  // Порівнюємо значення x
        assert_eq!(y, 2);  // Порівнюємо значення y
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}

