// Borrowing digunakan untuk mengakses nilai tanpa mengambil alih kepemilikan nilai tersebut.
// Mutable borrowing digunakan untuk mengakses nilai dengan mengizinkan perubahan nilai tersebut.
// Imutable borrowing digunakan untuk mengakses nilai tanpa mengizinkan perubahan nilai tersebut.

fn main() {
    println!("Contoh Deklarasi dan Penggunaan Borrowing");
    // 1. Peminjaman Referensi pada String
    let mut s1 = String::from("hello");
    let reference_to_s1 = &s1;
    println!("1. String Reference: {}", reference_to_s1);

    // 2. Peminjaman Referensi Mutable pada String
    let reference_to_mut_s1 = &mut s1;
    reference_to_mut_s1.push_str(", world!");
    println!("2. Mutable String Reference: {}", reference_to_mut_s1);

    // 3. Peminjaman Referensi pada Vector
    let v1 = vec![1, 2, 3];
    let reference_to_v1 = &v1;
    println!("3. Vector Reference: {:?}", reference_to_v1);

    // 4. Peminjaman Referensi Mutable pada Vector
    let mut v2 = vec![4, 5, 6];
    let reference_to_mut_v2 = &mut v2;
    reference_to_mut_v2.push(7);
    println!("4. Mutable Vector Reference: {:?}", reference_to_mut_v2);

    // 5. Peminjaman Referensi pada Tuple
    let tuple = (1, "hello", true);
    let reference_to_tuple = &tuple;
    println!("5. Tuple Reference: {:?}", reference_to_tuple);

    // 6. Peminjaman Referensi pada Struct
    #[derive(Debug)]
    struct MyStruct {
        data: i32,
        message: String,
    }
    let my_struct = MyStruct {
        data: 42,
        message: String::from("hello"),
    };
    let reference_to_struct = &my_struct;
    println!("6. Struct Reference: {:?}", reference_to_struct);

    // 7. Peminjaman Referensi Mutable pada Struct
    let mut my_struct2 = MyStruct {
        data: 24,
        message: String::from("world"),
    };
    let reference_to_mut_struct = &mut my_struct2;
    reference_to_mut_struct.message.push_str("!");
    println!("7. Mutable Struct Reference: {:?}", reference_to_mut_struct);

    // 8. Peminjaman Referensi pada Option
    let opt = Some(42);
    let reference_to_opt = &opt;
    println!("8. Option Reference: {:?}", reference_to_opt);

    // 9. Peminjaman Referensi pada Result
    let res: Result<i32, &str> = Ok(42);
    let reference_to_res = &res;
    println!("9. Result Reference: {:?}", reference_to_res);

    // 10. Peminjaman Borrowed String Slice
    let borrowed_str: &str = "hello";
    let reference_to_borrowed_str = &borrowed_str;
    println!(
        "10. Borrowed String Slice Reference: {:?}",
        reference_to_borrowed_str
    );
}
