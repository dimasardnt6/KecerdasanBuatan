# Cara Penggunaan dan Perbedaan Perintah `cargo run` dan `cargo build`

## 1. Perbedaan antara `cargo run` dan `cargo build`

- `cargo run`: Perintah ini akan mengompilasi proyek Anda jika perlu, lalu menjalankan outputnya (biasanya file yang dihasilkan bernama sama dengan proyek). Jadi, ini akan mengompilasi dan menjalankan kode Anda dalam satu langkah.

- `cargo build`: Perintah ini hanya akan mengompilasi proyek Anda menjadi executable (atau pustaka, tergantung jenis proyeknya) tanpa menjalankannya. Jadi, jika Anda hanya ingin mengompilasi kode tanpa menjalankannya, Anda dapat menggunakan `cargo build`.

## 2. Penjelasan Baris Kode di File `Cargo.toml`

- `[package]`: Ini adalah bagian yang berisi informasi tentang proyek Anda.
  - `name`: Nama proyek Anda.
  - `version`: Versi proyek Anda.
  - `authors`: Informasi tentang penulis proyek.
  - `edition`: Edisi Rust yang digunakan dalam proyek ini (dalam contoh ini, menggunakan edisi 2018).
- `[dependencies]`: Bagian ini digunakan untuk mendefinisikan dependensi atau pustaka eksternal yang diperlukan oleh proyek Anda. Saat ini, tidak ada dependensi yang ditentukan di sini.
