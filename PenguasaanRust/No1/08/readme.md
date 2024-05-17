# Ringkasan

A function in struct, enum, trait is called as a method.
impl Struct/Enum { // implement Struct or Enum
fn method_name(&self) -> type { // define a method
self.member // access the member variable
}}

1.  Define a trait method
    trait Trait_Name{ // define a trait
    fn trait_method(&self); // define a trait method
    }
2.  Implement the trait method
    impl Trait_Name for Struct/Enum{ // implement the trait
    fn trait_method(&self){ // implement the trait method
    self.member // access the member variable
    }}

    The feature of drop() method: “Last in, First out”.
    fn drop(&mut self){
    }

    1. Create a closure
       let closure_name = | parameter | { };
    2. Call the closure
       closure_name(parameter);
