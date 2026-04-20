#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data mahasiswa
#[contracttype]
#[derive(Clone, Debug)]
pub struct Mahasiswa {
    id: u64,
    nama: String,
    ipk: u32,         // IPK dikali 100, misal IPK 3.75 = 375 (hindari float)
    pendapatan: u64,  // Pendapatan orang tua dalam rupiah
}

// Struktur hasil ranking beasiswa
#[contracttype]
#[derive(Clone, Debug)]
pub struct HasilBeasiswa {
    peringkat: u32,
    mahasiswa: Mahasiswa,
    skor: u32,        // Skor akhir (lebih tinggi = lebih prioritas)
}

const MAHASISWA_DATA: Symbol = symbol_short!("MAH_DATA");
const MAX_PENERIMA: u32 = 3;

// Pendapatan maksimum referensi untuk normalisasi: 10.000.000
const PENDAPATAN_MAX: u64 = 10_000_000;

#[contract]
pub struct BeasiswaContract;

#[contractimpl]
impl BeasiswaContract {

    // Ambil semua data mahasiswa
    pub fn get_mahasiswa(env: Env) -> Vec<Mahasiswa> {
        return env.storage().instance().get(&MAHASISWA_DATA).unwrap_or(Vec::new(&env));
    }

    // Daftarkan mahasiswa baru
    pub fn daftar_mahasiswa(env: Env, nama: String, ipk: u32, pendapatan: u64) -> String {
        // Validasi IPK: maksimum 400 (setara 4.00)
        if ipk > 400 {
            return String::from_str(&env, "Error: IPK tidak valid, maksimum 4.00 (input: 400)");
        }

        let mut list: Vec<Mahasiswa> = env
            .storage()
            .instance()
            .get(&MAHASISWA_DATA)
            .unwrap_or(Vec::new(&env));

        let mhs = Mahasiswa {
            id: env.prng().gen::<u64>(),
            nama,
            ipk,
            pendapatan,
        };

        list.push_back(mhs);
        env.storage().instance().set(&MAHASISWA_DATA, &list);

        return String::from_str(&env, "Mahasiswa berhasil didaftarkan");
    }

    // Hapus mahasiswa berdasarkan id
    pub fn hapus_mahasiswa(env: Env, id: u64) -> String {
        let mut list: Vec<Mahasiswa> = env
            .storage()
            .instance()
            .get(&MAHASISWA_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..list.len() {
            if list.get(i).unwrap().id == id {
                list.remove(i);
                env.storage().instance().set(&MAHASISWA_DATA, &list);
                return String::from_str(&env, "Mahasiswa berhasil dihapus");
            }
        }

        return String::from_str(&env, "Mahasiswa tidak ditemukan");
    }

    // Hitung ranking dan tampilkan 3 penerima beasiswa
    pub fn get_penerima_beasiswa(env: Env) -> Vec<HasilBeasiswa> {
        let list: Vec<Mahasiswa> = env
            .storage()
            .instance()
            .get(&MAHASISWA_DATA)
            .unwrap_or(Vec::new(&env));

        let n = list.len();

        // Salin ke array sementara sebagai Vec<(skor, index)>
        // Skor dihitung: IPK_score + Pendapatan_score (masing-masing 0-500)
        // IPK score     = ipk * 500 / 400        → makin tinggi makin baik
        // Pendapatan score = (1 - pendapatan/MAX) * 500 → makin kecil makin baik

        // Karena no_std tidak ada sort bawaan, kita pakai selection sort manual
        // Simpan skor dan index asli
        let mut skor_list: Vec<u32> = Vec::new(&env);
        let mut idx_list: Vec<u32> = Vec::new(&env);

        for i in 0..n {
            let mhs = list.get(i).unwrap();

            // Skor IPK (0 - 500)
            let skor_ipk: u32 = (mhs.ipk as u32) * 500 / 400;

            // Skor pendapatan (0 - 500), makin kecil pendapatan makin tinggi skor
            let skor_pendapatan: u32 = if mhs.pendapatan >= PENDAPATAN_MAX {
                0
            } else {
                ((PENDAPATAN_MAX - mhs.pendapatan) * 500 / PENDAPATAN_MAX) as u32
            };

            let total_skor = skor_ipk + skor_pendapatan;

            skor_list.push_back(total_skor);
            idx_list.push_back(i);
        }

        // Selection sort descending berdasarkan skor
        for i in 0..n {
            let mut max_pos = i;
            for j in (i + 1)..n {
                if skor_list.get(j).unwrap() > skor_list.get(max_pos).unwrap() {
                    max_pos = j;
                }
            }
            // Swap skor
            if max_pos != i {
                let tmp_skor = skor_list.get(i).unwrap();
                let tmp_idx = idx_list.get(i).unwrap();

                skor_list.set(i, skor_list.get(max_pos).unwrap());
                idx_list.set(i, idx_list.get(max_pos).unwrap());

                skor_list.set(max_pos, tmp_skor);
                idx_list.set(max_pos, tmp_idx);
            }
        }

        // Ambil maksimal MAX_PENERIMA teratas
        let mut hasil: Vec<HasilBeasiswa> = Vec::new(&env);
        let batas = if n < MAX_PENERIMA { n } else { MAX_PENERIMA };

        for rank in 0..batas {
            let original_idx = idx_list.get(rank).unwrap();
            let mhs = list.get(original_idx).unwrap();
            let skor = skor_list.get(rank).unwrap();

            hasil.push_back(HasilBeasiswa {
                peringkat: rank + 1,
                mahasiswa: mhs,
                skor,
            });
        }

        return hasil;
    }
}

mod test;