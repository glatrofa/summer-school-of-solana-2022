/*
fn function_name() -> return_type {
    return value;
}

fn function_name() -> return_type {
    value
}
*/
#[allow(dead_code)]
pub fn fn_hello() -> &'static str {
    // the last command in a function without the ; is the return value
    "Hello from Barcelona"
}
