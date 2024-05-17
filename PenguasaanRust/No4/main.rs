mod helper; // Import module helper.rs

use helper::multiply; // Import fungsi multiply dari helper.rs

fn main() {
    let result = multiply(10, 3);
    println!("Hasil perkalian nya: {}", result);
}