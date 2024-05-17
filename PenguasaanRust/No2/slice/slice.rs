// Slicing umumnya digunakan untuk memanipulasi atau mengakses bagian 
//tertentu dari array, string, atau struktur data lainnya

fn main() {
    println!("Contoh Deklarasi dan Penggunaan Slice");
    // 1. Slice pada Array
    let arr = [1, 2, 3, 4, 5];
    let slice1 = &arr[1..3]; // Mengambil elemen dari indeks 1 hingga 2
    println!("1. Slice pada Array: {:?}", slice1);

    // 2. Mutable Slice pada Array
    let mut arr2 = [6, 7, 8, 9, 10];
    let slice2 = &mut arr2[2..4]; // Mengambil elemen dari indeks 2 hingga 3
    slice2[0] = 100; // Mengubah nilai pada indeks 2
    println!("2. Mutable Slice pada Array: {:?}", slice2);

    // 3. Slice pada Vector
    let vec = vec![1, 2, 3, 4, 5];
    let slice3 = &vec[1..4]; // Mengambil elemen dari indeks 1 hingga 3
    println!("3. Slice pada Vector: {:?}", slice3);

    // 4. Mutable Slice pada Vector
    let mut vec2 = vec![6, 7, 8, 9, 10];
    let slice4 = &mut vec2[2..5]; // Mengambil elemen dari indeks 2 hingga 4
    slice4[1] = 200; // Mengubah nilai pada indeks 3
    println!("4. Mutable Slice pada Vector: {:?}", slice4);

    // 5. Slice pada String
    let s = String::from("hello world");
    let slice5 = &s[2..5]; // Mengambil substring dari indeks 2 hingga 4
    println!("5. Slice pada String: {}", slice5);

    // 6. Mutable Slice pada String
    let mut s2 = String::from("Rust Programming");
    let slice6 = &mut s2[5..12]; // Mengambil substring dari indeks 5 hingga 11
    slice6.make_ascii_uppercase(); // Mengubah substring menjadi huruf besar
    println!("6. Mutable Slice pada String: {}", slice6);

    // 7. Slice pada Borrowed String Slice
    let borrowed_str: &str = "rust";
    let slice7 = &borrowed_str[1..3]; // Mengambil substring dari indeks 1 hingga 2
    println!("7. Slice pada Borrowed String Slice: {}", slice7);

    // 8. Mutable Slice pada Borrowed Mut String Slice
    let mut mut_borrowed_str = String::from("rust");
    let slice8 = &mut mut_borrowed_str[1..3]; // Mengambil substring dari indeks 1 hingga 2
    slice8.make_ascii_uppercase(); // Mengubah substring menjadi huruf besar
    println!(
        "8. Mutable Slice pada Borrowed Mut String Slice: {}",
        slice8
    );

    // 9. Slice pada Borrowed Vector
    let borrowed_vec: &[i32] = &[1, 2, 3, 4, 5];
    let slice9 = &borrowed_vec[1..3]; // Mengambil elemen dari indeks 1 hingga 2
    println!("9. Slice pada Borrowed Vector: {:?}", slice9);

    // 10. Mutable Slice pada Borrowed Mut Vector
    let mut mut_borrowed_vec = vec![6, 7, 8, 9, 10];
    let slice10 = &mut mut_borrowed_vec[2..4]; // Mengambil elemen dari indeks 2 hingga 3
    slice10[1] = 1000; // Mengubah nilai pada indeks 3
    println!("10. Mutable Slice pada Borrowed Mut Vector: {:?}", slice10);
}
