struct Circle { // create a struct type
    radius: f32, // struct member
}

impl Circle { // implement the struct
    fn area(&self) -> f32 { // define a method
        std::f32::consts::PI * self.radius * self.radius
    } // method body
}

fn main() {
    let obj = Circle { radius: 2000.00 }; // create a struct object
    println!("The Circle area is: {}", obj.area()); // call the method
}
