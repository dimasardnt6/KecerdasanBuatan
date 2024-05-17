fn main() {
    let s=String::from("R in 8 Hours"); // s owns “R in 8 Hours”
    let n=cal(s); // Warning! s will lose the ownership after used
    println!("Value of the string is: {}",s); // s is no longer available
    println!("Length of the string is: {}",n);
    }
    fn cal(s:String) -> usize {
    s.len() // get the length of the string
    }
    