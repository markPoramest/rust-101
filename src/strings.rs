pub fn strings() {
    let mut hello= String::from("Hello, ");
    hello.push_str("World!");

    print!("{}", hello);
    
    let mut name = "Bob".to_string();
    name.push_str(" February");
    println!("your
    name: {}", name);

    println!("your \
    name: {}", name);
    
    let first_name = "Aon".to_string();
    let last_name = "Saharat".to_string();
    println!("first_name: {}, last_name: {}", first_name, last_name);
}

