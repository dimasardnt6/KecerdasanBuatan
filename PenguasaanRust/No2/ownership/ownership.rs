// ownership adalah konsep kepemilikan

fn main() {
    println!("Contoh Deklarasi Ownership");
    // 1. String baru
    let mut s = String::from("hello");
    println!("1. String baru: {}", s);
    println!();

    // 2. Vektor baru
    let mut v = vec![1, 2, 3, 4, 5];
    println!("2. Vektor baru: {:?}", v);

    println!();

    println!("Contoh Penggunaan Ownership");
    // 1. Transfer kepemilikan
    let s1 = String::from("hello");
    let s2 = s1; // s1 tidak dapat diakses lagi karena kepemilikannya telah dipindahkan ke s2
                 // println!("{}", s1); // Error karena s1 tidak valid lagi
    println!("1. Transfer kepemilikan: s2 = {}", s2);

    // 2. Penggandaan mutable reference
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // error: hanya boleh satu mutable reference pada satu waktu

    // 3. Pengembalian nilai dari fungsi
    let s = create_string(); // kepemilikan dialihkan ke s
    println!("3. Pengembalian nilai dari fungsi: s = {}", s);

    // 4. Mengalihkan kepemilikan dengan fungsi
    let s = String::from("hello");
    consume_string(s.clone()); // kepemilikan dialihkan ke fungsi consume_string
    println!("4. Mengalihkan kepemilikan dengan fungsi: s = {}", s);

    // 5. Pemindahan kepemilikan dengan iterator
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone(); // v1 tidak dapat diakses lagi karena kepemilikannya telah dipindahkan ke v2
    println!("5. Pemindahan kepemilikan dengan iterator: v2 = {:?}", v2);

    // 6. Iterasi dan pemilikan
    let v = vec![1, 2, 3];
    for i in v {
        // i memiliki kepemilikan nilai-nilai dalam v
        println!("6. Iterasi dan pemilikan: i = {}", i);
    }
    // v tidak dapat diakses lagi karena nilai-nilainya telah dimiliki oleh i

    // 7. Fungsi yang mengembalikan tuple dengan kepemilikan
    let (s, num) = create_tuple(); // kepemilikan dialihkan ke s
    println!(
        "7. Fungsi yang mengembalikan tuple dengan kepemilikan: s = {}, num = {}",
        s, num
    );

    // 8. Koleksi di dalam struktur dengan kepemilikan
    let my_struct = MyStruct {
        s: String::from("hello"),
        num: 5,
    }; // kepemilikan string dimiliki oleh my_struct
    println!(
        "8. Koleksi di dalam struktur dengan kepemilikan: my_struct = {:?}",
        my_struct
    );

    // 9. Pemindahan kepemilikan melalui fungsi dengan struktur
    let my_struct = MyStruct {
        s: String::from("hello"),
        num: 5,
    };
    consume_struct(my_struct.clone()); // kepemilikan struktur dialihkan ke fungsi consume_struct
    println!(
        "9. Pemindahan kepemilikan melalui fungsi dengan struktur: my_struct = {:?}",
        my_struct
    );

    // 10. Penyalinan data ke struktur baru
    let my_struct = MyStruct {
        s: String::from("hello"),
        num: 5,
    };
    let new_struct = my_struct.clone(); // my_struct tidak dapat diakses lagi karena kepemilikannya telah dipindahkan ke new_struct
    println!(
        "10. Penyalinan data ke struktur baru: new_struct = {:?}",
        new_struct
    );
}

fn create_string() -> String {
    let s = String::from("hello");
    s
}

fn consume_string(s: String) {
    // s diambil alih di sini
}

fn create_tuple() -> (String, i32) {
    let s = String::from("hello");
    let num = 5;
    (s, num)
}

#[derive(Debug, Clone)]
struct MyStruct {
    s: String,
    num: i32,
}

fn consume_struct(my_struct: MyStruct) {
    // my_struct diambil alih di sini
}
