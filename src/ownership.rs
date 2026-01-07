pub fn ownershio() {
    // borrow and mutable
    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push_str(" world");
    println!("{}", r1);
    println!("{}", s);

    // 🧍 ข้อมูล = ของ
    // 🪪 owner = คนถือบัตร
    // 🖐️ borrow = ขอหยิบดู / ขอแก้ชั่วคราว
    //
    // มีบัตรได้ใบเดียว
    //
    // ถ้าให้คนอื่นถือ → ตัวเองใช้ไม่ได้
    //
    // ถ้ายืมอ่าน → ห้ามมีคนแก้
    //
    // ถ้ายืมแก้ → ต้องยืมคนเดียว
}
