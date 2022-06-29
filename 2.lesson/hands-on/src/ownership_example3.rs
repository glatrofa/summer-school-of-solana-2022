type Id = u32;

struct Handle(Id);

// ~ destructor
// it' called automatically when a variable runs out of scope
impl Drop for Handle {
    fn drop(&mut self) {
        println!("handle {} dropped!", self.0)
    }
}

pub fn main() {
    let handle_0 = Handle(0);
    // let Handle_2 = create_handle();
    let _ = create_handle(); // the value returned in '_' is dropped immediately
    Handle(3);
}

fn create_handle() -> Handle {
    Handle(1); // the scope start and ends here
    Handle(2) // this variable is returned
}

// stackL 1, 3, 2, 0
