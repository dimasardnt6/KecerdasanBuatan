fn main(){
    let num:i32 = 3; // given expression
    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"), // match this
        4 => println!("four"),
        _ => println!("something else"),
    }
}