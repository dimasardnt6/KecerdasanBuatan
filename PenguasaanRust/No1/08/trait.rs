/* contoh 1
struct Circle { // create a struct type
    radius: f32, // struct member
}

trait Calculate { // define a trait
    fn area(&self) -> f32; // define a trait method
}

impl Calculate for Circle { // implement the trait
    fn area(&self) -> f32 { // implement the trait method
        std::f32::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let obj = Circle { radius: 2000.00 }; // create a struct object
    println!("The Circle area is: {}", obj.area()); // call the method
}
*/

// contoh 2
pub trait Show { // define a trait
    fn show(&self); // define a trait method
}

impl<T> Show for T // implement the trait with generic
where
    T: ToString, // specify the String type
{
    fn show(&self) { // implement the trait method
        print!("{}", self.to_string());
    }
}

fn main() {
    String::from("C# in 8 Hours").show(); // call method
}
