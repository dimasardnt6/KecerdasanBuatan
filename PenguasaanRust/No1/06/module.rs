mod my_module{ // define a module
        pub fn test(){ // pub means public attribute
        println!("Hello My Friends!");
    }
}

fn main(){
    my_module::test(); // run the module
}