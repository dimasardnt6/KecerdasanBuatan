
// Variable binding
fn main(){
    println!("variable-binding");
    let (x, y) = (100, 200); 
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!();

    println!("Mutable Variable");
    let mut a = 100;
    let mut b = 200;
    a = a + 300;
    b = b + 400;
    println!("Finally a is {}", a);
    println!("Finally b is {}", b);
    println!();

    println!("String Assignment");
    let x = "hello".to_string();
    let y = String::from("hello");
    let z:&str = "hello";
    print!("{} {} {} ", x, y, z);
    println!();

    println!("Aritmetical Operator");
    println!("10 + 2 = {}", 10 + 2); // Tambah
    println!("10 - 2 = {}", 10 - 2); // Kurang
    println!("10 * 2 = {}", 10 * 2); // Kali
    println!("10 / 2 = {}", 10 / 2); // Bagi
    println!("10 % 2 = {}", 10 % 2); // Modulus
    println!();

    println!("Logical Operator");
    println!("true AND false is {}", true && false); // AND
    println!("true OR false is {}", true || false); // OR
    println!("NOT true is {}", ! true); // NOT
    println!();
        
    println!("Comparison Operator");
    let x:i32 = 100;
    let y:i32 = 200;
    println!("x is greater than y : {}", x > y); // Lebih besar
    println!("x is less than y : {}", x < y); // Lebih kecil
    println!("x is unequal to y : {}", x != y); // Tidak sama dengan
    println!("x is greater/equal to y : {}", x >= y); // Lebih besar sama dengan
    println!("x is less/equal to y : {}", x <= y); // Lebih kecil sama dengan
    println!("x is completely equal to y : {}", x == y); // Sama dengan
    println!();

    println!("Array");
    let mut a: [i32; 4] = [8; 4];
    a[1] = 10;
    a[2] = 20;
    a[3] = 10000;
    println!("{} {} {} {}",a[0], a[1], a[2], a[3]);
    println!();

    println!("Array");
    let a:[f32; 4] = [0.1, 0.2, 0.3, 0.4];
    println!("{} {} {} {}",a[0],a[1],a[2],a[3]);
    println!();
    
    println!("Slice");
    let a = [0, 10, 20, 30, 40, 50, 60];
    let slice = &a[2..5]; //ekstrak elemen ke 2 sampai 4
    println!("{}",slice[0] );
    println!("{}",slice[1] );
    println!("{}",slice[2] );
}

