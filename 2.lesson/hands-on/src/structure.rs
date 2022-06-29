struct Person {
    first_name: String,
    last_name: String,
}

// pub fn main() {
//     let p = Person {
//         first_name: "John".to_string(),
//         last_name: "Smith".to_string()
//     };
//     println!("Person {} {}", p.first_name, p.last_name);
// }

impl Person {
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string(),
        }
    }
}

pub fn main() {
    let p = Person::new("Janko", "Hrasko");
    println!("Person {} {}", p.first_name, p.last_name);
}
