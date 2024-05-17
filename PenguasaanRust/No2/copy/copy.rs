// Copy dalam rust digunakan untuk membuat salinan dari suatu nilai yang ada. 

fn main() {
    println!("Contoh Deklarasi dan Penggunaan Copy");
    // 1. Integer
    let x = 5;
    let y = x; // Nilai dari x disalin ke y
    println!("1. Integer: x = {}, y = {}", x, y);

    // 2. Boolean
    let a = true;
    let b = a; // Nilai dari a disalin ke b
    println!("2. Boolean: a = {}, b = {}", a, b);

    // 3. Karakter
    let c = 'c';
    let d = c; // Nilai dari c disalin ke d
    println!("3. Karakter: c = {}, d = {}", c, d);

    // 4. Tipe data Tuple
    let tuple = (1, true, 'a');
    let copied_tuple = tuple; // Nilai dari tuple disalin ke copied_tuple
    println!(
        "4. Tuple: tuple = {:?}, copied_tuple = {:?}",
        tuple, copied_tuple
    );

    // 5. Array dengan tipe data Copy
    let arr1 = [1, 2, 3];
    let arr2 = arr1; // Nilai dari arr1 disalin ke arr2
    println!("5. Array: arr1 = {:?}, arr2 = {:?}", arr1, arr2);

    // 6. Referensi ke tipe data Copy
    let s1 = "hello";
    let s2 = s1; // Nilai dari s1 disalin ke s2
    println!("6. String Slice: s1 = {}, s2 = {}", s1, s2);

    // 7. Option dengan tipe data Copy
    let opt1 = Some(42);
    let opt2 = opt1; // Nilai dari opt1 disalin ke opt2
    println!("7. Option: opt1 = {:?}, opt2 = {:?}", opt1, opt2);

    // 8. Result dengan tipe data Copy
    let res1: Result<i32, &str> = Ok(42);
    let res2 = res1; // Nilai dari res1 disalin ke res2
    println!("8. Result: res1 = {:?}, res2 = {:?}", res1, res2);

    // 9. Pointer ke tipe data Copy
    let ptr1 = &5;
    let ptr2 = ptr1; // Nilai dari ptr1 disalin ke ptr2
    println!("9. Pointer: ptr1 = {:p}, ptr2 = {:p}", ptr1, ptr2);

    // 10. Referensi ke tipe data Copy
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1; // Nilai dari vec1 disalin ke vec2
    println!("10. Reference: vec1 = {:?}, vec2 = {:?}", vec1, vec2);
}
