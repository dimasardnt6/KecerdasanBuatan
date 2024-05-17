# Pengantar Large Language Models dan Transformer Architecture - Pradeep Menon

## Model Bahasa Besar/ Large Language Model (LLM)

Model Bahasa Besar dilatih pada sejumlah besar data teks untuk menghasilkan teks yang koheren dan lancar. Contohnya adalah GPT yang memiliki miliaran parameter.

Premis dasar dari model bahasa adalah kemampuannya memprediksi kata atau sub-kata berikutnya berdasarkan teks yang telah diamati.

## Transformer Architecture: The Building Block

1. **Penyematan Input:** Kata-kata direpresentasikan sebagai angka untuk diproses oleh model.
2. **Pengkodean Posisi:** Setiap kata diberi angka yang mewakili posisinya dalam kalimat.
3. **Encoder:** Menerjemahkan teks input ke dalam representasi numerik yang menangkap makna dan konteks melalui lapisan-lapisan perhatian diri.
4. **Pelatihan Model:** Urutan output dipindahkan untuk memastikan decoder hanya melihat kata-kata sebelumnya, membantu dalam memprediksi kata-kata berikutnya.
5. **Penyematan Output:** Mengubah output menjadi format numerik dengan menggunakan fungsi kerugian yang membandingkan prediksi dengan nilai target.
6. **Inferensi:** Menggunakan representasi input dan output untuk menghasilkan teks bahasa alami sebagai output.
7. **Lapisan Linear:** Memetakan output ke ruang dimensi yang sesuai dengan input aslinya.
8. **Fungsi Softmax:** Menghasilkan distribusi probabilitas untuk setiap token output dalam kosakata.

## Konsep dari Attention Mechanism

Mekanisme attention memungkinkan model untuk fokus pada bagian yang berbeda dari urutan input saat membuat setiap token output. Ini membantu model dalam memahami hubungan antara input yang jauh satu sama lain dan menangani input dengan panjang yang berbeda.

## Kesimpulan

Model Bahasa Besar dengan arsitektur Transformer, seperti GPT, mampu menghasilkan teks yang koheren dan lancar berkat pelatihan pada data teks yang luas. Arsitektur Transformer memungkinkan proses prediksi yang lebih efisien dan mirip dengan pemahaman manusia, dengan fitur-fitur utama seperti penyematan input, encoder, decoder, mekanisme perhatian, dan kemampuan untuk menyesuaikan pendekatan tergantung pada panjang input.
