# Perbedaan membuat library dan file utama di rust

1. File Utama:

- File utama adalah tempat di mana Anda menulis kode yang ingin Anda jalankan sebagai program utama.
- File utama tempat di mana Anda mendefinisikan fungsi main(), yang akan dieksekusi saat program Anda dijalankan.
- Bayangkan file utama seperti "pintu masuk" ke aplikasi Anda. Jadi File utama adalah tempat pertama yang akan dijalankan ketika Anda memulai aplikasi Anda.

2. Library:

Membuat kumpulan kode yang dapat digunakan kembali oleh proyek-proyek lain.
Library biasanya terdiri dari beberapa fungsi, struktur data, atau fitur-fitur lain.
Menggunakan library dengan cara menambahkan dependensinya di Cargo.toml.
cargo new nama-proyek --lib
