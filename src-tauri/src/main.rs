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
let mut secret = [0u8; 32];
let input_bytes = password.trim().as_bytes();


if input_bytes.len() > 32 {
    return Err(tauri::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Password must be 32 bytes or less")));
}


secret[(32 - input_bytes.len())..].copy_from_slice(input_bytes);

    let nonce = &secret[0..12];

    let secret_generic: GenericArray<u8, U32> = GenericArray::clone_from_slice(&secret);
    let nonce_generic: GenericArray<u8, U12> = GenericArray::clone_from_slice(&nonce);

    let mut cipher = chacha20::ChaCha20::new(&secret_generic, &nonce_generic);


    let mut cipher_bytes = match bs58::decode(input_cipher.trim()).into_vec() {
    Ok(bytes) => bytes,
    Err(_) => return Err(tauri::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to decode base58 string")))
    };

    cipher.apply_keystream(&mut cipher_bytes);

    let minimal_seed = String::from_utf8_lossy(&cipher_bytes).to_string();

    let minimal_seed_array: Vec<String> = minimal_seed.chars().collect::<Vec<_>>().chunks(2).map(|chunk| chunk.iter().collect::<String>()).collect();

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

    let recovered_seed: Vec<String> = minimal_seed_array.iter().filter_map(|char| {
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
