//  Scoope digunakan untuk menentukan area dimana suatu variabel dapat diakses

fn main() {
    println!("Contoh Deklarasi dan Penggunaan Scope");
    // 1. Scope pada blok kode
    {
        let x = 5;
        println!("1. Scope pada blok kode: x = {}", x);
    }
    // println!("x = {}", x); // Error: x tidak dapat diakses di luar blok kode

    // 2. Scope pada fungsi
    let y = 10;
    scope_function(y);
    // println!("y = {}", y); // Error: y tidak dapat diakses di luar fungsi

    // 3. Scope pada loop
    for i in 0..3 {
        println!("3. Scope pada loop: i = {}", i);
    }
    // println!("i = {}", i); // Error: i tidak dapat diakses di luar loop

    // 4. Scope pada if statement
    let condition = true;
    if condition {
        let z = 15;
        println!("4. Scope pada if statement: z = {}", z);
    }
    // println!("z = {}", z); // Error: z tidak dapat diakses di luar if statement

    // 5. Scope pada match expression
    let number = 42;
    match number {
        1 => println!("5. Scope pada match: Satu"),
        42 => {
            let msg = "Empat puluh dua";
            println!("5. Scope pada match: {}", msg);
        },
        _ => println!("5. Scope pada match: Default"),
    }

    // 6. Scope pada enum variant
    enum MyEnum {
        Variant1(u32),
        Variant2(String),
    }
    let my_var = MyEnum::Variant1(5);
    match my_var {
        MyEnum::Variant1(val) => println!("6. Scope pada enum variant: Variant1({})", val),
        MyEnum::Variant2(str) => println!("6. Scope pada enum variant: Variant2({})", str),
    }

    // 7. Scope pada modul
    mod my_module {
        pub fn greet() {
            println!("7. Scope pada modul: Hello!");
        }
    }
    my_module::greet();

    // 8. Scope pada struktur
    struct MyStruct {
        x: i32,
    }
    let my_struct = MyStruct { x: 20 };
    println!("8. Scope pada struktur: x = {}", my_struct.x);

    // 9. Scope pada trait
    trait MyTrait {
        fn hello(&self);
    }
    impl MyTrait for MyStruct {
        fn hello(&self) {
            println!("9. Scope pada trait: Hello from MyTrait!");
        }
    }
    my_struct.hello();

    // 10. Scope pada blok unsafe
    unsafe {
        println!("10. Scope pada blok unsafe: Hello from unsafe!");
    }
}

fn scope_function(y: i32) {
    println!("2. Scope pada fungsi: y = {}", y);
}
