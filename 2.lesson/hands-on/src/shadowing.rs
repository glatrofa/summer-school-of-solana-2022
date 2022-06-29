#[allow(dead_code)]
pub fn print_string_obj() {
    let name = "AckeeBlockchain";
    println!("Hi from: {}", name);
    // the second declaration 'shadow' the first, so the last one is valid
    let name = name.len();
    println!("name changed to ... {}", name);
}
