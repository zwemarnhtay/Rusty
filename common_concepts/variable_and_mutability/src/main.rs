fn main() {
    //immutable_variable();
    mutable_variable();
}

// when we run below code, we got error[E0384]: cannot assign twice to immutable variable `x`
// due to variable is immutable
// fn immutable_variable() {
//     let x = "hello";
//     println!("x : {x}");
//     x = "HELLO";
//     println!("x : {x}");
// }

fn mutable_variable() {
    let mut x = "hello";
    println!("x : {x}");
    x = "hello this is mutable";
    println!("x : {x}");
}
