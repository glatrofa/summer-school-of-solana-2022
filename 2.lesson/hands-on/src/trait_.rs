trait Show {
    fn show(&self) -> String;
}

// how to implement this behaviour
impl Show for i32 {
    fn show(&self) -> String {
        format!("four-bytes {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eighty-bytes {}", self)
    }
}

pub fn main() {
    let answer: i32 = 42;
    let maybe_pi: f64 = 3.24;
    let s1: String = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);
}
