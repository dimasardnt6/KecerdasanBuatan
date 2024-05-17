// Mutable digunakan untuk membuat variabel yang dapat diubah nilainya setelah deklarasi.

fn main() {
    println!("Contoh Deklarasi dan Penggunaan Mutable");
    // 1. Mutable pada integer
    let mut x = 5;
    println!("1. Mutable pada integer (sebelum): {}", x);
    x = 10;
    println!("1. Mutable pada integer (setelah): {}", x);

    // 2. Mutable pada String
    let mut s = String::from("hello");
    println!("2. Mutable pada String (sebelum): {}", s);
    s.push_str(", world!");
    println!("2. Mutable pada String (setelah): {}", s);

    // 3. Mutable pada Vector
    let mut v = vec![1, 2, 3];
    println!("3. Mutable pada Vector (sebelum): {:?}", v);
    v.push(4);
    println!("3. Mutable pada Vector (setelah): {:?}", v);

    // 4. Mutable pada Tuple
    let mut tuple = (1, "hello");
    println!("4. Mutable pada Tuple (sebelum): {:?}", tuple);
    tuple.0 = 42;
    println!("4. Mutable pada Tuple (setelah): {:?}", tuple);

    // 5. Mutable pada Struct
    #[derive(Debug)]
    struct MyStruct {
        data: i32,
        message: String,
    }
    let mut my_struct = MyStruct {
        data: 5,
        message: String::from("world"),
    };
    println!("5. Mutable pada Struct (sebelum): {:?}", my_struct);
    my_struct.data = 10;
    println!("5. Mutable pada Struct (setelah): {:?}", my_struct);

    // 6. Mutable pada Option
    let mut opt = Some(42);
    println!("6. Mutable pada Option (sebelum): {:?}", opt);
    opt = None;
    println!("6. Mutable pada Option (setelah): {:?}", opt);

    // 7. Mutable pada Result
    let mut res: Result<i32, &str> = Ok(42);
    println!("7. Mutable pada Result (sebelum): {:?}", res);
    res = Err("error");
    println!("7. Mutable pada Result (setelah): {:?}", res);

    // 8. Mutable pada Borrowed String Slice
    let mut borrowed_str = "hello";
    println!(
        "8. Mutable pada Borrowed String Slice (sebelum): {}",
        borrowed_str
    );
    borrowed_str = "world";
    println!(
        "8. Mutable pada Borrowed String Slice (setelah): {}",
        borrowed_str
    );

    // 9. Mutable pada Borrowed Mut String Slice
    let mut mut_borrowed_str = String::from("hello");
    println!(
        "9. Mutable pada Borrowed Mut String Slice (sebelum): {}",
        mut_borrowed_str
    );
    let slice = &mut mut_borrowed_str;
    slice.push_str(", world!");
    println!(
        "9. Mutable pada Borrowed Mut String Slice (setelah): {}",
        mut_borrowed_str
    );

    // 10. Mutable pada Borrowed Vector
    let mut borrowed_vec = vec![1, 2, 3];
    println!(
        "10. Mutable pada Borrowed Vector (sebelum): {:?}",
        borrowed_vec
    );
    let vec_ref = &mut borrowed_vec;
    vec_ref.push(4);
    println!(
        "10. Mutable pada Borrowed Vector (setelah): {:?}",
        borrowed_vec
    );
}
