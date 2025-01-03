use std::io;

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

    // integer_types();
    // floating_points();
    // numeric_operations();
    tuples();
    array_type();

    arr_sample();
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

fn tuples(){
    let tup = ('A', "Hello", -555, 783.68, true);
    println!("{}", tup.0);

    let (_a, _b, c, _d, _e) = tup;
    println!("c : {c}");

    let hello = tup.1;
    println!("{hello}");
}

fn array_type(){
    let mut arr = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);

    let arr: [f64; 2]  = [2.1, 91.99];
    println!("{}", arr[1]);

    let str_arr = ["rust"; 3]; // => ["rust", "rust", "rust"]
    println!("{}", str_arr[2]);
}

fn arr_sample(){
    let arr = [1, 2, 3, 4, 5];

    println!("Enter an index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index must be a number");

    let element = arr[index];
    println!("The value of the element at index {index} is: {element}");
}