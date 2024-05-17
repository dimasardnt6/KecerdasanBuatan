# Ringkasan

1. Tentukan modul

```
mod nama_modul{
fungsi pub(){ }
}
```

2. Jalankan modul

```
nama_modul::fungsi();
```

3. Tentukan modul tertanam

```
mod m1{
mod m2{
fungsi pub(){ }
}
}
```

4. Jalankan modul tertanam
   m1::m2::fungsi();
   mod extern_file; // “mod “ memuat file eksternal
   gunakan extern_file::extern_fun; // “use” memuat fungsi eksternal
   Dalam bahasa Rust, semua fungsi bersifat pribadi secara default.
   Jika ada fungsi atau modul yang bersifat pribadi, maka dapat diakses melalui langsungnya modul induk atau modul itu sendiri.
   gunakan:: super:: fungsi_induk; // mengakses fungsi induk
