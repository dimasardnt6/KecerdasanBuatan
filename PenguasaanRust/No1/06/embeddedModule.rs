mod m1{
    pub fn a(){
    println!("m1 module");
    }
        pub mod m2{ // embedded module
            pub fn b(){
            println!("m2 module");
            }
        }
    }
    fn main(){
    m1::a();
    m1::m2::b(); // runs the embedded module & function
    }