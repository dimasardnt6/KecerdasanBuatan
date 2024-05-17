fn main() {
    let x = 3;
    match x {
        2...6 => println!("from 2 to 6"), // match from 2 to 6
        _ => println!("others"),
    }
}
