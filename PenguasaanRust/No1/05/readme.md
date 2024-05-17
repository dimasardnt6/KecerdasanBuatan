# Ringkasan

1. Buat sebuah struct
   struct Nama_Struktur {
   anggota1: type
   anggota2: type
   ......
   }

2. Inisialisasi struct
   let objek = Nama_Struktur {
   anggota1: nilai1,
   anggota2: nilai2,
   ......
   }

3. Akses anggota
   obyek. anggota
   (1) Tentukan enum
   enum Enum_Nama{
   anggota 1,
   anggota 2,
   ......
   }
   (2) Akses ke anggota
   Enum_Nama::anggota
   Tentang Kepemilikan:
   type pengikatan variabel memiliki sumber daya, itu disebut kepemilikan. Namun type sebuah pengikatan variabel keluar dari cakupan, ia melepaskan sumber daya, dan hilang kepemilikan.
   (1) Referensi suatu variabel. &variabel
   (2) Referensi parameter parameter: &type
