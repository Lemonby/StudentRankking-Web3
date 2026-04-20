# 🎓 Student Scholarship Selection System

A **Soroban (Stellar blockchain)** smart contract that transparently and automatically selects and ranks scholarship recipients. The system evaluates students based on two objective parameters and determines the **top 3 scholarship recipients**.

---

## 📋 Project Description

This project was built as part of a **Web3 Workshop** with the goal of developing a decentralized application on the Stellar blockchain using Soroban smart contracts. By leveraging blockchain technology, the scholarship selection process becomes **transparent**, **tamper-proof**, and **fully automated** without any intermediaries.

---

## ⚙️ Selection Parameters

The system uses two parameters to determine scholarship recipients:

| Parameter | Description | Weight |
|---|---|---|
| **GPA** | Higher is better (max. 4.00) | 0 – 500 points |
| **Parent's Income** | Lower income gets higher priority | 0 – 500 points |
| **Total Score** | Combined score of both parameters | 0 – 1000 points |

### Scoring Formula

```
GPA Score     = (gpa / 400) × 500
Income Score  = ((10,000,000 - income) / 10,000,000) × 500
Total Score   = GPA Score + Income Score
```

> **Note:** GPA is multiplied by 100 to avoid floating point (e.g. GPA 3.75 → input `375`). Parent's income is entered in Indonesian Rupiah (IDR).

---

## 🛠️ Tech Stack

- **Language:** Rust (`#![no_std]`)
- **Platform:** [Soroban](https://soroban.stellar.org/) – Stellar Smart Contracts
- **SDK:** `soroban-sdk`

---

## 📁 Project Structure

```
contracts/
└── notes/
    └── src/
        ├── lib.rs      # Main smart contract
        └── test.rs     # Unit tests
    ├── Cargo.toml
    └── Makefile
Cargo.toml
Cargo.lock
README.md
```

---

## 🚀 Contract Functions

### `get_mahasiswa(env) -> Vec<Mahasiswa>`
Returns the full list of all registered students.

### `daftar_mahasiswa(env, nama, ipk, pendapatan) -> String`
Registers a new student into the system.

**Parameters:**
- `nama` – Student's full name
- `ipk` – GPA multiplied by 100 (e.g. GPA 3.75 → `375`)
- `pendapatan` – Parent's monthly income (in IDR)

### `hapus_mahasiswa(env, id) -> String`
Removes a student record by their unique ID.

### `get_penerima_beasiswa(env) -> Vec<HasilBeasiswa>`
Calculates scores for all students and returns the **top 3** as scholarship recipients, complete with their rank and final score.

---

## 📊 Scoring Example

| Student | GPA | Income | GPA Score | Income Score | Total |
|---|---|---|---|---|---|
| Budi | 3.80 (input: 380) | IDR 2,000,000 | 475 | 400 | **875** |
| Ani | 3.50 (input: 350) | IDR 500,000 | 437 | 475 | **912** ✅ |
| Citra | 3.90 (input: 390) | IDR 8,000,000 | 487 | 100 | **587** |

In the example above, **Ani** ranks higher despite having a lower GPA, because her parent's income is significantly lower — making her more eligible for financial aid.

---

## 🔧 Getting Started

### Prerequisites
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

### Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/notes.wasm \
  --network testnet \
  --source <YOUR_SECRET_KEY>
```

### Invoke Contract Functions

```bash
# Register a student
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn daftar_mahasiswa \
  -- --nama "Budi Santoso" --ipk 380 --pendapatan 2000000

# Get scholarship recipients
soroban contract invoke \
  --id <CONTRACT_ID> \
  --fn get_penerima_beasiswa
```

---

## 📄 License

This project was built for educational purposes as part of a Web3 workshop. Feel free to use and modify it.

---

## Contrack ID

CBRKLRINKR4ULCRQ3Z5F2XHW6M4FWK7LXZYCU2EGXCK3Q2PU2H4YYUKO

---

## Images Stellar Expert
<img width="1917" height="901" alt="image" src="https://github.com/user-attachments/assets/63ad9c64-27db-4cad-b8ac-d1980aa3e117" />

---

> Built with ❤️ using Soroban Smart Contracts on the Stellar Blockchain

