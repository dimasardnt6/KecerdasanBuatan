// Enum

enum Language { //define an enum
    Js, //members
    Go,
    VB,
}

fn program(var: Language) {
    match var {
        Language::Js => println!("JS in 8 Hours"),
        Language::Go => println!("Go in 8 Hours"),
        Language::VB => println!("VB in 8 Hours"),
    }
}

fn main() {
    program(Language::Js); //access the members
    program(Language::Go);
    program(Language::VB);
}