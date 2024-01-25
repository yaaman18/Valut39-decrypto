# Decrypto Tauri Application

## Overview
The Decrypto application, built with Tauri and Rust, is designed to decrypt and recover seed phrases encrypted using the ChaCha20 cipher algorithm. It efficiently handles the decryption process and converts minimalized seed phrases back to their original format.

## Features
- **Decryption of Data**: Decrypts data encrypted with the ChaCha20 cipher.
- **Recovery of Seed Phrases**: Converts minimalized seed phrases back to their standard format.
- **Secure Hashing**: Uses SHA-256 for hashing the password securely.

## Usage
- `handle_data`: Decrypts the input cipher with the provided password and recovers the seed phrase.

## How to Run
1. Ensure Rust and Tauri dependencies are installed.
2. Clone the repository.
3. Execute `cargo build` to compile the application.
4. Run the application.

## Dependencies
- ChaCha20 for decryption.
- SHA-256 for hashing.
- bs58 for Base58 encoding and decoding.
- Tokio for asynchronous programming.

## License
Copyright © 2024 Solidity Materials Co., Ltd. All Rights Reserved.

## Disclaimer
This software is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

---

# Decrypto Tauri アプリケーション

## 概要
このDecryptoアプリケーションは、TauriとRustで構築され、ChaCha20暗号アルゴリズムを使用して暗号化されたシードフレーズを復号化し、回復するように設計されています。効率的に復号化プロセスを処理し、簡略化されたシードフレーズを元の形式に戻します。

## 特徴
- **データの復号化**: ChaCha20暗号で暗号化されたデータを復号化します。
- **シードフレーズの回復**: 簡略化されたシードフレーズを標準的な形式に戻します。
- **セキュアなハッシング**: パスワードを安全にハッシュ化するためにSHA-256を使用します。

## 使い方
- `handle_data`: 提供されたパスワードで入力された暗号を復号化し、シードフレーズを回復します。

## 実行方法
1. RustおよびTauriの依存関係がインストールされていることを確認します。
2. リポジトリをクローンします。
3. `cargo build`を実行してアプリケーションをコンパイルします。
4. アプリケーションを実行します。

## 依存関係
- 復号化のためのChaCha20。
- ハッシュ化のためのSHA-256。
- Base58エンコーディングとデコーディングのためのbs58。
- 非同期プログラミングのためのTokio。

## ライセンス
Copyright © 2024 Solidity Materials Co., Ltd. All Rights Reserved.

## 免責事項
このソフトウェアは「現状のまま」提供されており、商品性、特定目的への適合性、および権利侵害を含め、明示的または暗黙的ないかなる保証も伴いません。作者または著作権所有者は、契約、不法行為、またはその他の形態にかかわらず、このソフトウェアの使用またはその他の取引から生じるいかなるクレーム、損害、その他の責任についても責任を負いません。
