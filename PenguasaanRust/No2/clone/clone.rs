//clone digunakan untuk membuat salinan dari suatu nilai yang ada

fn main() {
    println!("Contoh Deklarasi dan Penggunaan clone()");
    // 1. String
    let original_string = String::from("hello");
    let cloned_string = original_string.clone();
    println!(
        "1. String: Original: {}, Clone: {}",
        original_string, cloned_string
    );

    // 2. Vektor
    let original_vector = vec![1, 2, 3];
    let cloned_vector = original_vector.clone();
    println!(
        "2. Vektor: Original: {:?}, Clone: {:?}",
        original_vector, cloned_vector
    );

    // 3. Tuple
    let original_tuple = (5, String::from("world"));
    let cloned_tuple = original_tuple.clone();
    println!(
        "3. Tuple: Original: {:?}, Clone: {:?}",
        original_tuple, cloned_tuple
    );

    // 4. Struktur
    #[derive(Clone, Debug)]
    struct MyStruct {
        data: i32,
        message: String,
    }
    let original_struct = MyStruct {
        data: 42,
        message: String::from("hello"),
    };
    let cloned_struct = original_struct.clone();
    println!(
        "4. Struktur: Original: {:?}, Clone: {:?}",
        original_struct, cloned_struct
    );

    // 5. Array
    let original_array = [1, 2, 3, 4, 5];
    let cloned_array = original_array.clone();
    println!(
        "5. Array: Original: {:?}, Clone: {:?}",
        original_array, cloned_array
    );

    // 6. Option
    let original_option = Some("hello");
    let cloned_option = original_option.clone();
    println!(
        "6. Option: Original: {:?}, Clone: {:?}",
        original_option, cloned_option
    );

    // 7. Result
    let original_result: Result<i32, &str> = Ok(42);
    let cloned_result = original_result.clone();
    println!(
        "7. Result: Original: {:?}, Clone: {:?}",
        original_result, cloned_result
    );

    // 8. Box
    let original_box = Box::new(42);
    let cloned_box = original_box.clone();
    println!(
        "8. Box: Original: {:?}, Clone: {:?}",
        original_box, cloned_box
    );

    // 9. String Slice
    let original_str_slice = "hello";
    let cloned_str_slice = original_str_slice.to_string();
    println!(
        "9. String Slice: Original: {}, Clone: {}",
        original_str_slice, cloned_str_slice
    );

    // 10. Cow
    use std::borrow::Cow;
    let original_cow: Cow<str> = Cow::Borrowed("hello");
    let cloned_cow = original_cow.clone();
    println!("10. Cow: Original: {}, Clone: {}", original_cow, cloned_cow);
}
