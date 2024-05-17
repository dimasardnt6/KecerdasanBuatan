fn main(){
    let x = String::from("try"); // x owns “try”
    let y = x; // Warning! The ownership of x moves to y
    println!("{}", x); // Error! x is no longer available
    }
    