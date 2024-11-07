//ch10.1(3)
// Define a struct Point with a generic type T
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}
#[test]
fn cp10_1() {
    // Create an instance of Point with integers
    let _integer = Point { x: 5, y: 10 };

    // Create an instance of Point with floating point numbers
    let _float = Point { x: 1.0, y: 4.0 };

    // Print success message
    println!("Success!");
}

//ch10.2(2)
#[allow(dead_code)]
struct Array<T, const N: usize> {
    data: [T; N],
}
#[test]
fn cp10_2() {
    let _array_int = Array {
        data: [1, 2, 3],
    };

    let _array_float = Array {
        data: [1.0, 2.0, 3.0],
    };

    let _array_short = Array {
        data: [1, 2],
    };

    // Якщо хочете зробити щось з цими масивами, можете додати обробку тут.

    println!("Success!");
}

//ch10.3(6)
struct Sheep;
struct Cow;

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// Return a trait object (`Box<dyn Animal>`)
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

#[test]
fn cp10_3() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

//ch10.4(2)
trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    #[allow(dead_code)]
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    #[allow(dead_code)]
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}
#[test]
fn cp10_4() {
    // Use a vector to store trait objects, so we can have multiple types in the same collection
    let birds: Vec<Box<dyn Bird>> = vec![
        Box::new(Duck),
        Box::new(Swan),
    ];

    for bird in birds {
        bird.quack();
        // Since the birds are trait objects, we can't call `fly` directly because only `Duck` and `Swan` have the method `fly`.
        // bird.fly(); // This would cause an error because `bird` is of type `dyn Bird` and doesn't have a `fly` method.
    }
}

//ch10.5(4)
trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

// Реалізація ознак для CSStudent
impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}
#[test]
fn cp10_5() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string(),
    };

    // Викликаємо функцію з комп'ютерним студентом
    println!("{}", comp_sci_student_greeting(&student));
}





