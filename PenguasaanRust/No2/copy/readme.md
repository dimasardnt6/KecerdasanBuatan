# Copy

Dalam Rust, tipe data yang diimplementasikan dengan sifat Copy adalah tipe data yang diletakkan pada stack dan tidak memiliki penunjuk heap. Saat nilai dari tipe data ini disalin, nilai yang sama akan disalin ke lokasi memori yang berbeda, tanpa mempengaruhi nilai asli. Ini berbeda dengan tipe data yang mengimplementasikan sifat Clone, di mana nilai disalin secara eksplisit dengan membuat salinan mendalam dari data.
