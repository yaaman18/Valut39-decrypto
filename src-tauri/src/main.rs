// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::error::Error;
use std::env;
use std::path::PathBuf;

use chacha20::cipher::{KeyIvInit, StreamCipher};
use generic_array::GenericArray;
use generic_array::typenum::{U12, U32};
use tokio::fs::read_to_string;


#[tauri::command]
async fn handle_data(input_cipher: String, password: String) -> tauri::Result<String> {
    println!("Received input_cipher: {}", input_cipher);
    println!("Received password: {}", password);

    // recoverSeed関数を呼び出して、復元されたシードフレーズを取得
    let seed_phrase = match recoverSeed(input_cipher, password).await {
        Ok(result) => result,
        Err(e) => {
            return Err(e.into());
        }
    };
    Ok(seed_phrase)
}

async fn string_to_32_byte_array(str: &str) -> Result<[u8; 32], Box<dyn Error>> {
    let mut byte_array = [0u8; 32];
    let encoded = str.as_bytes();

    if encoded.len() > 32 {
        return Err("String too long to convert to 32-byte array".into());
    }

    byte_array[0..encoded.len()].copy_from_slice(encoded);
    Ok(byte_array)
}

async fn recoverSeed(input_cipher: String, password: String) -> tauri::Result<String> {
    // 暗号の復号化処理
    let secret_buff = password.trim().as_bytes();
    let mut padded_buf = [0u8; 12];
    padded_buf[(12 - secret_buff.len())..].copy_from_slice(secret_buff);
    let nonce = &padded_buf[0..12];

    let mut secret = [0u8; 32];
    secret.copy_from_slice(password.trim().as_bytes());

    let secret_generic: GenericArray<u8, U32> = GenericArray::clone_from_slice(&secret);
    let nonce_generic: GenericArray<u8, U12> = GenericArray::clone_from_slice(&nonce);

    let mut cipher = chacha20::ChaCha20::new(&secret_generic, &nonce_generic);


    let mut cipher_bytes = match bs58::decode(input_cipher.trim()).into_vec() {
    Ok(bytes) => bytes,
    Err(_) => return Err(tauri::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to convert to UTF-8 string")))
    };
    cipher.apply_keystream(&mut cipher_bytes);

    let minimal_seed = match String::from_utf8(cipher_bytes) {
    Ok(str) => str,
    Err(_) => return Err(tauri::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to convert to UTF-8 string"))),
    };

    let minimal_seed_array: Vec<&str> = minimal_seed.match_indices(" ").map(|(i, _)| &minimal_seed[0..i]).collect();

    // wordlist_enとwordlist_minimalを読み込む
    let mut path_en = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_en.push("src");
    path_en.push("resources");
    path_en.push("wordlist_en.txt");

    let mut path_minimal = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_minimal.push("src");
    path_minimal.push("resources");
    path_minimal.push("wordlist_minimal.txt");

    let wordlist_en = read_to_string(&path_en).await?;
    let wordlist_minimal = read_to_string(&path_minimal).await?;

    let wordlist_en: Vec<String> = wordlist_en.lines().map(|s| s.to_owned()).collect();
    let wordlist_minimal: Vec<String> = wordlist_minimal.lines().map(|s| s.to_owned()).collect();

    let en_to_index: HashMap<_, _> = wordlist_en.iter().enumerate().map(|(i, word)| (word, i)).collect();
    let minimal_to_index: HashMap<_, _> = wordlist_minimal.iter().enumerate().map(|(i, word)| (word, i)).collect();

    let recovered_seed: Vec<String> = minimal_seed_array.iter().filter_map(|&char| {
    let index = match en_to_index.get(&char.to_string()) {
        Some(index) => *index,
        None => {
            eprintln!("Unknown word: {}", char); // エラーメッセージを標準エラー出力に出力
            return None;
        }
    };
    Some(wordlist_en[index].clone())
}).collect();

let seed_phrase = recovered_seed.join(" ");

    Ok(seed_phrase)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
