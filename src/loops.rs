pub fn loops() {
    let mut number = 1;
    loop {
        number += 2;
        if number > 10 {
            println!("{}", number);
            break;
        }
    }

    let mut number = 3;
    let new_number = loop {
        number -= 1;
        if number == 0 {
            break number;
        }
    };

    println!("{}", new_number);

    number = 100;
    while number != 0 {
        println!("{}!", number);
        number -= 10;
    }
    println!("LIFTOFF!!!");

    for i in 0..10 {
        println!("{}", i);
    }

    for i in (0..10).step_by(5) {
        print!("{} ", i);
    }
    println!();

    let arr = [1, 2, 3, 4, 5];
    for element in arr {
        print!("{} ", element);
    }
}
