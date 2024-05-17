// Struct"
struct Member{ // create a struct
    id: i32, // member1: type
    name: String,
    working: bool,
}

struct Square { // create a struct
    len: i32, 
    wid: i32,
}

fn main()  {
    let clerk = Member { // initialize the struct 
        id: 016320,
        name: "Anto".to_string(),
        working: true,
    };
    println!("ID: {}", clerk.id); // access the members
    println!("Name: {}", clerk.name);
    println!("Working: {}", clerk.working);

    let table = Square { len: 10, wid: 8 }; // initialization
    println!("The area is {}", table.len * table.wid); // access
}