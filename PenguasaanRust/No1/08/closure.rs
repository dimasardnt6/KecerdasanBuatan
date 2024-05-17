/* contoh 2
fn main() {
    let my_closure = |num: i32| { num + 200 }; // create a closure
    let num = 100; // define a variable

    // Call the closure and print the result
    println!("{}", my_closure(num));
}
*/

fn main() {
    let mut capacity = "Hard disk capacity: 5000".to_string();
    {
        let mut my_closure = |c: char| { capacity.push(c) }; // closure
        my_closure('G'); // call the closure
    }
    println!("{:?}", capacity); // {:?} is used to output a string
}

