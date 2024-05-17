fn main() {
    let x = 7;
    match x {
        var @ 2...5 => println!("{}", var), // binding
        _ => println!("others"),
    }
}
