fn main() {
    // PART SEVEN BORROWING

    let mut  a = String::from("hello");
    some_function(&mut a);

    println!("function:{}", a);

    // ** PART One
    // let mut x = 10;
    // const y: i32 = 20;
    // println!("my variable value is: {}", x);
    // x = 11;
    // println!("my variable value is: x={},y={}", x, y);

    // let a: u8 = 255;
    // let b: i64 = -2500;
    // let c = 0xFF;
    // let d = 0o77;
    // let e = 0b1111_00000;
    // println!("u8:{}", a);
    // println!("i64:{}", b);
    // println!("c:{}", c);
    // println!("d:{}", d);
    // println!("e:{}", e);
}

fn some_function(input: &mut String) {
    input.push_str(" afshin");
    print!("some function value : {}", input);
}
