use std::io;
use std::io::Read;

pub fn stdio() {
    println!("knock knock!");
    let mut name = String::new();
    io::stdin().read_line(&mut name);

    println!("name: {}", name);
}

