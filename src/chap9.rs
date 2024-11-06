//ch9(3)
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `&self` to fill in the blank.
    pub fn show_state(&self) {
        println!("The current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }
}

#[test]
fn cp9() {
    let mut traffic_light = TrafficLight {
        color: "red".to_string(),
    };

    traffic_light.show_state(); // Output: The current state is red
    traffic_light.change_state();
    traffic_light.show_state(); // Output: The current state is green

    println!("Success!");
}
