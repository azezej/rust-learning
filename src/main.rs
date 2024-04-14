use std::array;

fn variables_and_constants() {
    // definition
    let x = 10.0;
    println!("{}", x);

    // mutability
    // let y = 12.5; y = 15.0; <- throws an error, not mutable
    let mut y: f64 = 10.0;
    y = 15.0; // is ok

    // shadowing
    let t = 10;
    let t = t + 10;
    println!("{}", t);

    // scope
    let v = 30;
    {
        let v = 40;
        println!("inner v is: {}", v)
    }
    println!("v is {v}");

    // constants need a type and capital letters
    const MAX_VALUE: i32 = 100;
}

fn main() {
    variables_and_constants();
    let unsigned_num: u8 = 5; //unsigned
    let signed_num: i8 = 5; //iSIZE int size of
    let float_num: f32 = 5.0; // float (f64)
    let arch_1: usize = 5; // pointer sized unsigned integer
    let arch_2: isize = 5; // pointer sized signed integer
    let char: char = 'a'; // one character 8bits
    let b: bool = true; // or false.. xd

    // type aliasing ^___^ rust is weird
    type Age = u8;
    let your_age: Age = 14;

    // type conversion "as" keyword
    let a: i32 = 10;
    let b = a as f64;

    // &str and String
    let fixed_str: &str = "Fixed length string";
    let mut flexible_str: String = String::from("This string may grow");

    // Arrays
    let mut array_1: [i32; 5] = [4, 2, 3, 5, 9]; // one type, fixed size
    let num = array_1[3];
    println!("{:?}", array_1); // "{:?}" for compound data types? Compound data types are made up of more than one component.

    let array_2 = [0; 10]; // 10 items of value 0

    let vec_1: Vec<i32> = vec![1, 2, 3, 4, 5];
}
