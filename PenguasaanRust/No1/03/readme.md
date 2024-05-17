# Ringkasan

1. Sintaks pengikatan variabel:

   ```
   let (var1, var2) = (value1, value2);
   let mut variable = value;
   let x = “hello”.to_string(); // convert text to a string
   let y = String::from("hello"); // get text directly
   let z:&str = "hello";
   ```

2. Rust memiliki operator seperti: operator aritmatika, operator logika,
   operator perbandingan,

3. Sintaks untuk membuat array:
   ```
   let mut array: [type; length] = [default; length];
   let array: [type; length] = [val1, val2, val3……];
   The syntax to create a slice is as follows:
   let slice = &array[ start..last-1];
   ```
