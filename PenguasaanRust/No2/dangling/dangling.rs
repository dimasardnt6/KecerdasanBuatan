// Dangling digunakan untuk menunjukkan variabel yang telah dihapus atau tidak dapat diakses lagi.

fn main() {
    println!("Contoh Deklarasi dan Penggunaan Dangling");
    // 1. Dangling pada String
    let dangling_string = dangling_string();
    // println!("1. Dangling pada String: {}", dangling_string); // Error: String telah dihapus

    // 2. Dangling pada Vector
    let dangling_vector = dangling_vector();
    // println!("2. Dangling pada Vector: {:?}", dangling_vector); // Error: Vector telah dihapus

    // 3. Dangling pada Struct
    let dangling_struct = dangling_struct();
    // println!("3. Dangling pada Struct: {:?}", dangling_struct); // Error: Struct telah dihapus

    // 4. Dangling pada Option
    let dangling_option = dangling_option();
    // println!("4. Dangling pada Option: {:?}", dangling_option); // Error: Option telah dihapus

    // 5. Dangling pada Result
    let dangling_result = dangling_result();
    // println!("5. Dangling pada Result: {:?}", dangling_result); // Error: Result telah dihapus

    // 6. Dangling pada Borrowed String Slice
    let dangling_borrowed_str = dangling_borrowed_str();
    // println!("6. Dangling pada Borrowed String Slice: {}", dangling_borrowed_str); // Error: Borrowed String Slice telah dihapus

    // 7. Dangling pada Borrowed Mut String Slice
    let dangling_mut_borrowed_str = dangling_mut_borrowed_str();
    // println!("7. Dangling pada Borrowed Mut String Slice: {}", dangling_mut_borrowed_str); // Error: Borrowed Mut String Slice telah dihapus

    // 8. Dangling pada Borrowed Vector
    let dangling_borrowed_vec = dangling_borrowed_vec();
    // println!("8. Dangling pada Borrowed Vector: {:?}", dangling_borrowed_vec); // Error: Borrowed Vector telah dihapus

    // 9. Dangling pada Borrowed Mut Vector
    let dangling_mut_borrowed_vec = dangling_mut_borrowed_vec();
    // println!("9. Dangling pada Borrowed Mut Vector: {:?}", dangling_mut_borrowed_vec); // Error: Borrowed Mut Vector telah dihapus

    // 10. Dangling pada Borrowed Reference
    let dangling_borrowed_ref = dangling_borrowed_ref();
    // println!("10. Dangling pada Borrowed Reference: {:?}", dangling_borrowed_ref); // Error: Borrowed Reference telah dihapus
}

fn dangling_string() -> String {
    let s = String::from("dangling");
    s // String akan dihapus di sini
}

fn dangling_vector() -> Vec<i32> {
    let v = vec![1, 2, 3];
    v // Vector akan dihapus di sini
}

fn dangling_struct() -> MyStruct {
    #[derive(Debug)]
    struct MyStruct {
        data: i32,
    }
    let s = MyStruct { data: 42 };
    s // Struct akan dihapus di sini
}

fn dangling_option() -> Option<i32> {
    let opt = Some(42);
    opt // Option akan dihapus di sini
}

fn dangling_result() -> Result<i32, &str> {
    let res: Result<i32, &str> = Ok(42);
    res // Result akan dihapus di sini
}

fn dangling_borrowed_str() -> &'static str {
    let s: &'static str = "dangling";
    s // Borrowed String Slice akan dihapus di sini
}

fn dangling_mut_borrowed_str() -> &'static mut str {
    let mut s: &'static mut str = String::from("dangling").as_mut_str();
    s // Borrowed Mut String Slice akan dihapus di sini
}

fn dangling_borrowed_vec() -> &'static Vec<i32> {
    let v: &'static Vec<i32> = &vec![1, 2, 3];
    v // Borrowed Vector akan dihapus di sini
}

fn dangling_mut_borrowed_vec() -> &'static mut Vec<i32> {
    let mut v: &'static mut Vec<i32> = &mut vec![1, 2, 3];
    v // Borrowed Mut Vector akan dihapus di sini
}

fn dangling_borrowed_ref() -> &'static i32 {
    let x: &'static i32 = &42;
    x // Borrowed Reference akan dihapus di sini
}
