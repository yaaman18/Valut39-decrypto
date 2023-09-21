// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::error::Error;
use std::env;
use std::path::PathBuf;
use std::io::{self, Write};

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
    let mut byte_array = [0u8; 32];  // ゼロで初期化
    let encoded = str.as_bytes();

    if encoded.len() > 32 {
        return Err("String too long to convert to 32-byte array".into());
    }

    byte_array[0..encoded.len()].copy_from_slice(encoded);
    Ok(byte_array)
}


async fn recoverSeed(input_cipher: String, password: String) -> tauri::Result<String> {

// パスワードを32バイトの配列に変換
let mut secret = [0u8; 32];  // 32バイトのゼロで初期化された配列
let input_bytes = password.trim().as_bytes();

// 入力が32バイト以上の場合はエラー
if input_bytes.len() > 32 {
    return Err(tauri::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Password must be 32 bytes or less")));
}

// 入力を32バイトの配列にコピー
secret[(32 - input_bytes.len())..].copy_from_slice(input_bytes);


    // nonceの生成（最初の12バイトを使用）
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


    // 8. minimal_seedを2文字ごとに分割してminimal_seed_arrayに格納
    let minimal_seed_array: Vec<String> = minimal_seed.chars().collect::<Vec<_>>().chunks(2).map(|chunk| chunk.iter().collect::<String>()).collect();



    println!("minimal_seed: {}", minimal_seed);
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
    let minimal_to_index: HashMap<_, _> = wordlist_minimal.iter().enumerate().map(|(i, word)| (word.as_str(), i)).collect();

  // minimal_seed_arrayのデバッグ出力
println!("minimal_seed_array: {:?}", minimal_seed_array);

// minimal_to_indexのデバッグ出力
println!("minimal_to_index: {:?}", minimal_to_index);
    // 9. minimal_seed_arrayの各要素をwordlistMinimalで検索し、対応するwordlistEnの単語で置き換え
    let recovered_seed: Vec<String> = minimal_seed_array.iter().filter_map(|char| {
    let index = match minimal_to_index.get(&char[..]){  // &Stringを&strに変更
        Some(index) => *index,
        None => {
            eprintln!("Unknown word: {}", char); // エラーメッセージを標準エラー出力に出力
            return None;
        }
        };
        println!("Recovered word: {}", wordlist_en[index]);
        Some(wordlist_en[index].clone())
    }).collect();
    println!("recovered_seed: {:?}", recovered_seed);

    // 10. 置き換えた単語をスペースで連結して、通常のシードフレーズを生成
    let seed_phrase = recovered_seed.join(" ");
    println!("Recovered seed phrase: {}", seed_phrase);
    Ok(seed_phrase)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");





//         let mut input_cipher = String::new();
//         let mut password = String::new();

//     print!("暗号文を入力してください: ");
// if let Err(e) = io::stdout().flush() {
//     eprintln!("An error occurred while flushing stdout: {}", e);
//     return;
// }
// if let Err(e) = io::stdin().read_line(&mut input_cipher) {
//     eprintln!("An error occurred while reading from stdin: {}", e);
//     return;
// }

// print!("パスワードを入力してください: ");
// if let Err(e) = io::stdout().flush() {
//     eprintln!("An error occurred while flushing stdout: {}", e);
//     return;
// }
// if let Err(e) = io::stdin().read_line(&mut password) {
//     eprintln!("An error occurred while reading from stdin: {}", e);
//     return;
// }

// // 改行文字を削除
// input_cipher = input_cipher.trim().to_string();
// password = password.trim().to_string();

// // 非同期関数を呼び出すためにtokioランタイムを使用
// let runtime = match tokio::runtime::Runtime::new() {
//     Ok(rt) => rt,
//     Err(e) => {
//         eprintln!("Failed to create Tokio runtime: {}", e);
//         return;
//     }
// };

// let seed_phrase = match runtime.block_on(recoverSeed(input_cipher, password)) {
//     Ok(phrase) => phrase,
//     Err(e) => {
//         eprintln!("An error occurred while recovering the seed: {}", e);
//         return;
//     }
// };

//     println!("復元されたシードフレーズ: {}", seed_phrase);
}
