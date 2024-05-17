// 1. Struct untuk menyimpan informasi buku
struct Book {
    title: String,
    author: String,
}

impl Book {
    fn new(title: &str, author: &str) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
        }
    }

    fn print_info(&self) {
        println!("Title: {}, Author: {}", self.title, self.author);
    }
}

// 2. Struct untuk menyimpan informasi hewan peliharaan
struct Pet {
    name: String,
    species: String,
}

impl Pet {
    fn new(name: &str, species: &str) -> Self {
        Pet {
            name: name.to_string(),
            species: species.to_string(),
        }
    }

    fn print_info(&self) {
        println!("Name: {}, Species: {}", self.name, self.species);
    }
}

// 3. Struct untuk menyimpan informasi mobil
struct Car {
    make: String,
    model: String,
}

impl Car {
    fn new(make: &str, model: &str) -> Self {
        Car {
            make: make.to_string(),
            model: model.to_string(),
        }
    }

    fn print_info(&self) {
        println!("Make: {}, Model: {}", self.make, self.model);
    }
}

// 4. Struct untuk menyimpan informasi pengguna
struct User {
    username: String,
    email: String,
}

impl User {
    fn new(username: &str, email: &str) -> Self {
        User {
            username: username.to_string(),
            email: email.to_string(),
        }
    }

    fn print_info(&self) {
        println!("Username: {}, Email: {}", self.username, self.email);
    }
}

// 5. Struct untuk menyimpan informasi film
struct Movie {
    title: String,
    director: String,
}

impl Movie {
    fn new(title: &str, director: &str) -> Self {
        Movie {
            title: title.to_string(),
            director: director.to_string(),
        }
    }

    fn print_info(&self) {
        println!("Title: {}, Director: {}", self.title, self.director);
    }
}

// 6. Struct untuk menyimpan informasi kota
struct City {
    name: String,
    country: String,
}

impl City {
    fn new(name: &str, country: &str) -> Self {
        City {
            name: name.to_string(),
            country: country.to_string(),
        }
    }

    fn print_info(&self) {
        println!("City: {}, Country: {}", self.name, self.country);
    }
}

// 7. Struct untuk menyimpan informasi produk
struct Product {
    name: String,
    price: f64,
}

impl Product {
    fn new(name: &str, price: f64) -> Self {
        Product {
            name: name.to_string(),
            price,
        }
    }

    fn print_info(&self) {
        println!("Product: {}, Price: ${}", self.name, self.price);
    }
}

// 8. Struct untuk menyimpan informasi anggota keluarga
struct FamilyMember {
    name: String,
    relation: String,
}

impl FamilyMember {
    fn new(name: &str, relation: &str) -> Self {
        FamilyMember {
            name: name.to_string(),
            relation: relation.to_string(),
        }
    }

    fn print_info(&self) {
        println!("Name: {}, Relation: {}", self.name, self.relation);
    }
}

// 9. Struct untuk menyimpan informasi komputer
struct Computer {
    brand: String,
    processor: String,
}

impl Computer {
    fn new(brand: &str, processor: &str) -> Self {
        Computer {
            brand: brand.to_string(),
            processor: processor.to_string(),
        }
    }

    fn print_info(&self) {
        println!("Brand: {}, Processor: {}", self.brand, self.processor);
    }
}

// 10. Struct untuk menyimpan informasi akun bank
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, balance: f64) -> Self {
        BankAccount {
            owner: owner.to_string(),
            balance,
        }
    }

    fn print_info(&self) {
        println!("Owner: {}, Balance: ${}", self.owner, self.balance);
    }
}

fn main() {
    println!("Contoh pembuatan dan pemanggilan struct sederhana\n");

    println!("Contoh 1");
    let book = Book::new("The Rust Programming Language", "Steve Klabnik and Carol Nichols");
    book.print_info();
    println!();

    println!("Contoh 2");
    let pet = Pet::new("Buddy", "Dog");
    pet.print_info();
    println!();

    println!("Contoh 3");
    let car = Car::new("Toyota", "Corolla");
    car.print_info();
    println!();

    println!("Contoh 4");
    let user = User::new("johndoe", "john@example.com");
    user.print_info();
    println!();

    println!("Contoh 5");
    let movie = Movie::new("Inception", "Christopher Nolan");
    movie.print_info();
    println!();

    println!("Contoh 6");
    let city = City::new("Paris", "France");
    city.print_info();
    println!();

    println!("Contoh 7");
    let product = Product::new("Laptop", 999.99);
    product.print_info();
    println!();

    println!("Contoh 8");
    let family_member = FamilyMember::new("Alice", "Sister");
    family_member.print_info();
    println!();

    println!("Contoh 9");
    let computer = Computer::new("Apple", "M1");
    computer.print_info();
    println!();

    println!("Contoh 10");
    let bank_account = BankAccount::new("John Doe", 1500.00);
    bank_account.print_info();
    println!();
}
