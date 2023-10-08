// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::env;
use std::io;

use chacha20::cipher::{KeyIvInit, StreamCipher};
use generic_array::GenericArray;
use generic_array::typenum::{U12, U32};
use sha2::{Digest, Sha256};

const WORDLIST_EN: &str = include_str!("resources/wordlist_en.txt");
const WORDLIST_MINIMAL: &str = include_str!("resources/wordlist_minimal.txt");


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

    // 以下のコードを修正して、バイナリに埋め込まれたワードリストを利用する
    let wordlist_en: Vec<String> = WORDLIST_EN.lines().map(|s| s.to_owned()).collect();
    let wordlist_minimal: Vec<String> = WORDLIST_MINIMAL.lines().map(|s| s.to_owned()).collect();
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
