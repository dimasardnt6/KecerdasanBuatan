# Ringkasan

1. Metode pertama untuk membuat vektor
   misalkan nama_vektor = vec! [val1, val2, val3……];

2. Metode kedua untuk membuat vektor
   misalkan v = vec![val; mengulang];

3. Metode ketiga untuk membuat vektor
   let mut v=Vec::baru(); // gunakan kata kunci “baru”.
   v.push('value') // tetapkan nilai ke vektor
   Anda dapat menggunakan | untuk mencocokkan beberapa pola.
   pola1 | pola2
   Simbol “…” dapat cocok dengan nilai dalam rentang yang ditentukan:
   nomor1… nomor2
   @ dapat mengikat variabel ke suatu rentang:
   variabel @ rentang
   Pustaka standar Rust menyediakan Opsi untuk obat generik:
   Opsi enum<T> {
   Beberapa (T),
   Tidak ada,
   }
