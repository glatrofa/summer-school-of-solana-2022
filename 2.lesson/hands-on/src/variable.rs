#[allow(dead_code)]
pub fn print_string_obj() {
    // mut let variables be mutable
    let mut txt_fees = 25_000;
    println!("Txt fees is {} ", txt_fees);
    txt_fees = 35_000;
    println!("Txt fees changed to {}", txt_fees);
}
