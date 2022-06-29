// similar to python
// copy of data i never made, they are borrowed

pub fn main() {
    let ints: [i32; 5] = [1, 2, 3, 4, 5];
    let slice1: &[i32] = &ints[0..2];
    let slice2: &[i32] = &ints[1..]; // open range

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}

