use crate::option;

fn name(id: u32) -> Option<String> {
    if id == 1 {
        Some("Mark".to_string())
    } else {
        None
    }
}

pub fn option() {
    match name(1) {
        Some(x) => println!("{}", x),
        None => {println!("None")}
    }

    if let Some(x) = name(0) {
        println!("{}", x);
    } else {
        println!("None");
    }

    let name = name(0).unwrap_or("Jedi".to_string());
    println!("{}", name);
}

