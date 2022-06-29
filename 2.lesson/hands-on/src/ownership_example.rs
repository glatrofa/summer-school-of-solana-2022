// fn main(){
//     let v = vec![1,2,3];
//     let v2 = v; // new owner of v is v2
//     display(v2); // we are not sending the ownership of v2
//     println("In main {:?}", v2);
// }

// fn display(v:Vec<i32>){
//     println!("inside display {:?}", v);
// }

// correct version:
pub fn main(){
    let v = vec![1,2,3];
    let v2 = v;
    let v2_return = display(v2);
    println!("In main {:?}", v2_return);
}

fn display(v:Vec<i32>) -> Vec<i32>{
    println!("inside display {:?}", v);
    return v;
}
