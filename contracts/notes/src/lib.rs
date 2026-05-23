#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Env,
    String,
    Symbol,
    Vec,
};

// Struktur data hasil MBTI
#[contracttype]
#[derive(Clone, Debug)]
pub struct MBTIResult {
    id: u64,
    name: String,
    mbti_type: String,
    description: String,
}

// Storage key
const MBTI_DATA: Symbol = symbol_short!("MBTI_DATA");

#[contract]
pub struct MBTIContract;

#[contractimpl]
impl MBTIContract {

    // Ambil semua hasil MBTI
    pub fn get_results(env: Env) -> Vec<MBTIResult> {

        env.storage()
            .instance()
            .get(&MBTI_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Menentukan MBTI
    pub fn determine_mbti(
        env: Env,
        name: String,
        e: u32,
        i: u32,
        s: u32,
        n: u32,
        t: u32,
        f: u32,
        j: u32,
        p: u32,
    ) -> String {

        // Ambil data lama
        let mut results: Vec<MBTIResult> = env.storage()
            .instance()
            .get(&MBTI_DATA)
            .unwrap_or(Vec::new(&env));

        // Menentukan tiap huruf MBTI
        let ei = if e >= i { "E" } else { "I" };
        let sn = if s >= n { "S" } else { "N" };
        let tf = if t >= f { "T" } else { "F" };
        let jp = if j >= p { "J" } else { "P" };

        // Membuat string MBTI manual
        let mbti_str = if ei == "E" && sn == "S" && tf == "T" && jp == "J" {
            "ESTJ"
        } else if ei == "E" && sn == "S" && tf == "T" && jp == "P" {
            "ESTP"
        } else if ei == "E" && sn == "S" && tf == "F" && jp == "J" {
            "ESFJ"
        } else if ei == "E" && sn == "S" && tf == "F" && jp == "P" {
            "ESFP"
        } else if ei == "E" && sn == "N" && tf == "T" && jp == "J" {
            "ENTJ"
        } else if ei == "E" && sn == "N" && tf == "T" && jp == "P" {
            "ENTP"
        } else if ei == "E" && sn == "N" && tf == "F" && jp == "J" {
            "ENFJ"
        } else if ei == "E" && sn == "N" && tf == "F" && jp == "P" {
            "ENFP"
        } else if ei == "I" && sn == "S" && tf == "T" && jp == "J" {
            "ISTJ"
        } else if ei == "I" && sn == "S" && tf == "T" && jp == "P" {
            "ISTP"
        } else if ei == "I" && sn == "S" && tf == "F" && jp == "J" {
            "ISFJ"
        } else if ei == "I" && sn == "S" && tf == "F" && jp == "P" {
            "ISFP"
        } else if ei == "I" && sn == "N" && tf == "T" && jp == "J" {
            "INTJ"
        } else if ei == "I" && sn == "N" && tf == "T" && jp == "P" {
            "INTP"
        } else if ei == "I" && sn == "N" && tf == "F" && jp == "J" {
            "INFJ"
        } else {
            "INFP"
        };

        let mbti = String::from_str(&env, mbti_str);

        // Deskripsi
        let description = match mbti_str {
            "INTJ" => "Strategic and logical thinker",
            "INFP" => "Idealistic and creative",
            "ENTP" => "Innovative and energetic",
            "ESFJ" => "Friendly and caring",
            _ => "Unique personality type",
        };

        // Object hasil
        let result = MBTIResult {
            id: env.prng().gen::<u64>(),
            name,
            mbti_type: mbti.clone(),
            description: String::from_str(&env, description),
        };

        // Simpan
        results.push_back(result);

        env.storage()
            .instance()
            .set(&MBTI_DATA, &results);

        mbti
    }

    // Hapus hasil MBTI
    pub fn delete_result(env: Env, id: u64) -> String {

        let mut results: Vec<MBTIResult> = env.storage()
            .instance()
            .get(&MBTI_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..results.len() {

            if results.get(i).unwrap().id == id {

                results.remove(i);

                env.storage()
                    .instance()
                    .set(&MBTI_DATA, &results);

                return String::from_str(
                    &env,
                    "Hasil MBTI berhasil dihapus"
                );
            }
        }

        String::from_str(
            &env,
            "Data tidak ditemukan"
        )
    }
}

mod test;