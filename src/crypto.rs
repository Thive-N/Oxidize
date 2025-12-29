use crate::PathBuf;
use crate::oxide::{clean_paths, read_oxide_file};
use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use argon2::Argon2;
use std::collections::HashSet;

const SALT: &[u8] = b"EXAMPLE_SALT_42";
const NONCE: &[u8; 12] = b"EXAMPLENONCE";

fn generate_key_from_password(password: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();
    argon2
        .hash_password_into(password.as_bytes(), SALT, &mut key)
        .expect("Failed to generate key from password");
    key
}

fn generate_target_files(files: Vec<PathBuf>) -> HashSet<PathBuf> {
    let raw_files: Vec<PathBuf> = if files.is_empty() {
        read_oxide_file().expect("Failed to read .oxide file")
    } else {
        files
    };
    clean_paths(raw_files)
}
fn generate_key(password: Option<String>) -> [u8; 32] {
    let pwd = match password {
        Some(p) => p,
        _ => rpassword::prompt_password("enter password> ").unwrap(),
    };
    generate_key_from_password(&pwd)
}

pub fn encrypt(files: Vec<PathBuf>, password: Option<String>, keep: bool) {
    let target_files = generate_target_files(files);
    let key = generate_key(password);
    for file in target_files {
        let data = std::fs::read(&file).expect("Failed to read file");
        let encrypted_data = encrypt_data(&key, &data);
        std::fs::write(&file, &encrypted_data).expect("Failed to write encrypted data");
    }
}

pub fn decrypt(files: Vec<PathBuf>, password: Option<String>, keep: bool) {
    let target_files = generate_target_files(files);
    let key = generate_key(password);
    for file in target_files {
        let data = std::fs::read(&file).expect("Failed to read file");
        let decrypted_data = decrypt_data(&key, &data).expect("Failed to decrypt data");
        std::fs::write(&file, &decrypted_data).expect("Failed to write decrypted data");
    }
}

fn encrypt_data(key: &[u8; 32], data: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key.into());
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(NONCE), data)
        .expect("encryption failure!");
    ciphertext
}
fn decrypt_data(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, aes_gcm::aead::Error> {
    let cipher = Aes256Gcm::new(key.into());
    let ciphertext = cipher
        .decrypt(Nonce::from_slice(NONCE), data)
        .expect("decryption failure!");
    Ok(ciphertext)
}
