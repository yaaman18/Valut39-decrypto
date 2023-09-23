// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::error::Error;
use std::env;
use std::path::PathBuf;
use std::io;

use chacha20::cipher::{KeyIvInit, StreamCipher};
use generic_array::GenericArray;
use generic_array::typenum::{U12, U32};
use tokio::fs::read_to_string;
use sha2::{Digest, Sha256};


#[tauri::command]
async fn handle_data(input_cipher: String, password: String) -> tauri::Result<String> {
    println!("Received input_cipher: {}", input_cipher);
    println!("Received password: {}", password);

    let seed_phrase = match recover_seed(input_cipher, password).await {
    Ok(result) => result,
    Err(e) => {
        return Err(e.into());
    }
    };
    Ok(seed_phrase)
}

async fn recover_seed(input_cipher: String, password: String) -> tauri::Result<String> {
    // パスワードをSHA-256でハッシュ化
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();

    let secret: GenericArray<u8, U32> = GenericArray::clone_from_slice(&result);
    let nonce_buff = [0u8; 12];
    let nonce: GenericArray<u8, U12> = GenericArray::clone_from_slice(&nonce_buff);

    let mut cipher = chacha20::ChaCha20::new(&secret, &nonce);

    let mut cipher_bytes = match bs58::decode(input_cipher.trim()).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Err(tauri::Error::from(io::Error::new(io::ErrorKind::InvalidData, "Failed to decode base58 string")))
    };

    cipher.apply_keystream(&mut cipher_bytes);

    let recovered_seed = String::from_utf8_lossy(&cipher_bytes).to_string();

    // 以下のコードはそのまま（wordlistの処理など）
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
    let minimal_to_index: HashMap<_, _> = wordlist_minimal.iter().enumerate().map(|(i, word)| (word.as_str(), i)).collect();

    let recovered_seed: Vec<String> = recovered_seed.chars().collect::<Vec<_>>().chunks(2).map(|chunk| chunk.iter().collect::<String>()).collect::<Vec<String>>().iter().filter_map(|char| {
        let index = match minimal_to_index.get(&char[..]){
            Some(index) => *index,
            None => {
                eprintln!("Unknown word: {}", char);
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
