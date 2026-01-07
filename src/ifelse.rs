pub fn ifelse() {
    let number1 = 134;
    let number2 = 17;

    if number1 > number2 {
        println!("{} is greater than or equal to {}", number1, number2);
    } else if (number1 < number2) {
        println!("{} is less than or equal to {}", number1, number2);
    } else {
        println!("{} is greater than or equal to {}", number1, number2);
    }
}
