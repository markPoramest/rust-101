use crate::option;


// 🔹 รู้ขนาดตอน compile → ใช้ array
// 🔹 ขนาดเปลี่ยน / รับจาก input → ใช้ Vec
pub fn vec() {
    let mut vec1 = vec!["Beer"];
    vec1.push("Faii");

    println!("{:?}", vec1);

    match vec1.pop() {
        Some(s) => println!("{}", s),
        None => println!("None"),
    }

    if let Some(s) = vec1.pop() {
        println!("{}", s);
    } else {
        println!("None");
    }

    let res = vec1.pop().unwrap_or("No Data");
    println!("{}", res);

    let res = vec1.pop().unwrap();
    println!("{}", res); // panic


}

