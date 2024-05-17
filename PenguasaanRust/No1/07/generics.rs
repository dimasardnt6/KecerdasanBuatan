fn main() {
    let x: Option<bool> = Some(true); // generic parameters
    let y: Option<i32> = Some(10);
    let z: Option<f64> = Some(20.88);
    let n: Option<i32> = None;
    match x {
        Some(x) => {
            println!("x = {}", x)
        }
        None => {
            println!("x = None")
        }
    }
    match y {
        Some(y) => {
            println!("y = {}", y)
        }
        None => {
            println!("y = None")
        }
    }
    match z {
        Some(z) => {
            println!("z = {}", z)
        }
        None => {
            println!("z = None")
        }
    }
    match n {
        Some(n) => {
            println!("n = {}", n)
        }
        None => {
            println!("n = None")
        }
    }
}
