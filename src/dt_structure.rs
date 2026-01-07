use crate::init_var;

pub fn tuple() {
    let student_a = ("Beer", "A", 3.76);
    let student_b = ("Aon","B", 3.17);

    let name_student_a = student_a.0;
    println!("name student a: {}", name_student_a);

    let (name_student_b, grade_student_b, score_student_b) = student_b;
    println!("student b: {}, {}, {}", name_student_b, grade_student_b, score_student_b);
}

pub fn arr() {
    let students = ["Linda", "Aon", "Beer", "Paul"];
    println!("students: {:?}", students);

    println!("student first: {}", students[0]);
}

pub fn slice() {
    let arr = ["Beer", "Aon", "B", "Paul"];
    let slice = &arr[1..3];

    println!("slice: {:?}", slice); // Aon Beer

    let mut arr2 = [1, 2, 3, 4, 5];
    let slice = &mut arr2[1..3];

    slice[0] += 2;
    println!("slice: {:?}", slice);
}
