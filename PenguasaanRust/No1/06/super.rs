mod sup_module { // parent module
    fn a() -> i32 {
        100
    }

    pub mod sub_module { // child module
        use super::a; // access parent function a
        pub fn b() {
            println!("{}", a()); // calls parent function a
        }
    }
}

fn main() {
    sup_module::sub_module::b(); // call function b
}
