fn main() {
    println!("Vektor1");
    let v = vec![100, 200, 300, 400]; // create a vector
    println!("First element is :{}", v[0]); // access the first element
    println!("Second element is :{}", v[1]);
    println!("Third element is :{}", v[2]);
    println!("Fourth element is :{}", v[3]);
    println!();

    println!("Vektor2");
    let x = vec![8; 3];
    println!("First element is :{}", x[0]);
    println!("Second element is :{}", x[1]);
    println!("Third element is :{}", x[2]);
    println!();

    println!("Vektor3");
    let mut y = Vec::new(); // create a vector
    y.push('R'); // set R as the first element of vector
    y.push('U');
    y.push('B');
    y.push('Y');
    for n in y {
        print!("{}", n);
    }
}
