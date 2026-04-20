# 🎓 Sistem Seleksi Beasiswa Mahasiswa

Smart contract berbasis **Soroban (Stellar blockchain)** untuk menyeleksi dan meranking penerima beasiswa secara transparan dan otomatis. Sistem mengevaluasi mahasiswa berdasarkan dua parameter objektif dan menentukan **3 penerima beasiswa terbaik**.

---

## 📋 Deskripsi Proyek

Proyek ini dibuat dalam rangka **Workshop Web3** dengan tujuan membangun aplikasi decentralized di atas blockchain Stellar menggunakan Soroban smart contract. Dengan memanfaatkan teknologi blockchain, proses seleksi beasiswa menjadi **transparan**, **tidak dapat dimanipulasi**, dan **otomatis** tanpa perantara.

---

## ⚙️ Parameter Seleksi

Sistem menggunakan dua parameter untuk menentukan penerima beasiswa:

| Parameter | Keterangan | Bobot |
|---|---|---|
| **IPK** | Semakin tinggi semakin baik (maks. 4.00) | 0 – 500 poin |
| **Pendapatan Orang Tua** | Semakin kecil semakin prioritas | 0 – 500 poin |
| **Total Skor** | Gabungan kedua parameter | 0 – 1000 poin |

### Formula Scoring

```
Skor IPK         = (ipk / 400) × 500
Skor Pendapatan  = ((10.000.000 - pendapatan) / 10.000.000) × 500
Total Skor       = Skor IPK + Skor Pendapatan
```

> **Catatan:** IPK diinput dikali 100 untuk menghindari float (IPK 3.75 → input `375`). Pendapatan orang tua diinput dalam satuan rupiah.

---

## 🛠️ Teknologi

- **Bahasa:** Rust (`#![no_std]`)
- **Platform:** [Soroban](https://soroban.stellar.org/) – Smart Contract Stellar
- **SDK:** `soroban-sdk`

---

## 📁 Struktur Proyek

```
contracts/
└── notes/
    └── src/
        ├── lib.rs      # Smart contract utama
        └── test.rs     # Unit test
    ├── Cargo.toml
    └── Makefile
Cargo.toml
Cargo.lock
README.md
```

---

## 🚀 Fungsi Kontrak

### `get_mahasiswa(env) -> Vec<Mahasiswa>`
Mengambil seluruh daftar mahasiswa yang telah terdaftar.

### `daftar_mahasiswa(env, nama, ipk, pendapatan) -> String`
Mendaftarkan mahasiswa baru ke dalam sistem.

**Parameter:**
- `nama` – Nama lengkap mahasiswa
- `ipk` – IPK dikali 100 (contoh: IPK 3.75 → `375`)
- `pendapatan` – Pendapatan orang tua per bulan (dalam rupiah)

### `hapus_mahasiswa(env, id) -> String`
Menghapus data mahasiswa berdasarkan ID unik.

### `get_penerima_beasiswa(env) -> Vec<HasilBeasiswa>`
Menghitung skor seluruh mahasiswa dan mengembalikan **3 mahasiswa teratas** sebagai penerima beasiswa, lengkap dengan peringkat dan skor akhir.

---

## 📊 Contoh Perhitungan

| Mahasiswa | IPK | Pendapatan | Skor IPK | Skor Pendapatan | Total |
|---|---|---|---|---|---|
| Budi | 3.80 (input: 380) | Rp 2.000.000 | 475 | 400 | **875** |
| Ani | 3.50 (input: 350) | Rp 500.000 | 437 | 475 | **912** ✅ |
| Citra | 3.90 (input: 390) | Rp 8.000.000 | 487 | 100 | **587** |

Dari contoh di atas, **Ani** mendapat peringkat lebih tinggi meskipun IPK-nya lebih rendah, karena pendapatan orang tuanya jauh lebih kecil.

---

## 🔧 Cara Menjalankan

### Prasyarat
- [Rust](https://www.rust-lang.org/tools/install)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test
```

### Deploy ke Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/notes.wasm \
  --network testnet \
  --source <YOUR_SECRET_KEY>
```

### Memanggil Fungsi

```bash
# Daftarkan mahasiswa
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn daftar_mahasiswa \
  -- --nama "Budi Santoso" --ipk 380 --pendapatan 2000000

# Lihat penerima beasiswa
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn get_penerima_beasiswa
```

---

## 📄 Lisensi

Proyek ini dibuat untuk keperluan edukasi dalam rangka workshop Web3. Bebas digunakan dan dimodifikasi.

## Contrack ID

CBRKLRINKR4ULCRQ3Z5F2XHW6M4FWK7LXZYCU2EGXCK3Q2PU2H4YYUKO

---

> Dibuat dengan ❤️ menggunakan Soroban Smart Contract – Stellar Blockchain
