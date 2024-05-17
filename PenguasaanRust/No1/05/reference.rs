fn main() {
    let s=String::from("R in 8 Hours");
    let n=cal(&s); // reference
    println!("Value of the string is: {}",s);
    println!("Length of the string is: {}",n);
    }
    fn cal(s:&String) -> usize { // reference
    s.len() // get the length of the string
    }