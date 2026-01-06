const GLOBAL_STR: &str = "123";

pub fn init_var() {
    println!("Hello, world!");
    print!("Hello, world!\n");
    print!("Hello, world!\n");

    let hello: &str = "Hello, string";
    let number1 = 1234;
    let number2 = 98765;

    println!("{} {}", hello, number1);
    println!("Math {} + {} = {}", number1, number2, number1 + number2);

    let mut hello_mutable = "mutable word";
    println!("{}", hello_mutable);
    hello_mutable = hello;

    println!("{}", hello_mutable);

    const CON_NUMBER:i32 = 16;

    println!("{}", CON_NUMBER);
    println!("{}", GLOBAL_STR);

    // Suffix
    let number64 = 1223_i64; // integer 64
    let float64 = 1463_f64; // float 64

    println!("{}", number64);
    println!("{}", float64);
}
