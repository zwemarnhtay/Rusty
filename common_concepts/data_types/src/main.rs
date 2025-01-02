fn main() {
    let a: &str = "this is string";  // or we can use ' let a = "this is string" '
    println!("{a}");

    let b: char = 'A';
    let rust_emoji = '🦀';
    print!("{b} is char ");
    println!("{rust_emoji}");

    let t = false;
    let f: bool = true;
    println!("{t} & {f}");

    integer_types();
    floating_points();
    numeric_operations();
}

fn integer_types(){
    let a = 5; // default => i32 (4 bytes)
    println!("size of a in bytes: {}", std::mem::size_of_val(&a));

    let b: u8 = 3; // u8 (1 bytes)
    println!("size of b in bytes: {}", std::mem::size_of_val(&b));
    let c: u8 = 2;
    println!("{b} + {c} = {}", b + c);

    let x: i8 = 12;
    let y: i8 = 15;
    let ans: i8 = x - y;
    println!("ans: {ans}");
}


fn floating_points(){
    let a = 3.0; // default => f64 (8 bytes)
    println!("size of a in bytes: {}", std::mem::size_of_val(&a));

    let b: f32 = 3.4; // f32 (4 bytes)
    println!("size of b in bytes: {}", std::mem::size_of_val(&b));
    let c = 2.3;
    println!("3.4 + 2.3 = {}", b + c);
}

fn numeric_operations(){
    let mut sum = 5 + 2;
    println!("sum is {sum} & size of sum in bytes {}", std::mem::size_of_val(&sum));

    sum = 5i8 + 2;
    println!("sum1 is {sum} & size of sum1 in bytes {}", std::mem::size_of_val(&sum));

    let sub = 3.0 - 4.2501; // we can't minus from integer to float
    println!("subtraction is {sub} & size of sub in bytes {}", std::mem::size_of_val(&sub));

    let sub2 : f32 = 3.0 - 4.2501;
    println!("subtraction is {sub2} & size of sub in bytes {}", std::mem::size_of_val(&sub2));

    let mul = 2 * 77777;
    println!("multiplication is {mul} & size of mul in bytes {}", std::mem::size_of_val(&mul));


    let a = 5 / 2;
    println!("division : {a}");

    let b : f32 = 3.61 / 4.1;
    println!("division : {b}");

    let reminder = 5 % 2;
    println!("reminder : {reminder}")
}
