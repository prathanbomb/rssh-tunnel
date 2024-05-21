use std::fmt;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use chacha20poly1305::{
    aead::{Aead, NewAead},
    ChaCha20Poly1305, Key, Nonce,
};
use hex::{decode, encode};
use rand_core::{OsRng, RngCore};

const NONCE_SIZE: usize = 12;

#[derive(Debug)]
pub enum CryptoError {
    Argon2Error(argon2::password_hash::Error),
    HexError(hex::FromHexError),
    Utf8Error(std::string::FromUtf8Error),
    AeadError(chacha20poly1305::aead::Error),
    InvalidDataFormat,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::Argon2Error(e) => write!(f, "Argon2 error: {}", e),
            CryptoError::HexError(e) => write!(f, "Hex decoding error: {}", e),
            CryptoError::Utf8Error(e) => write!(f, "UTF-8 conversion error: {}", e),
            CryptoError::AeadError(e) => write!(f, "AEAD encryption/decryption error: {}", e),
            CryptoError::InvalidDataFormat => write!(f, "Invalid data format"),
        }
    }
}

impl From<argon2::password_hash::Error> for CryptoError {
    fn from(err: argon2::password_hash::Error) -> CryptoError {
        CryptoError::Argon2Error(err)
    }
}

impl From<hex::FromHexError> for CryptoError {
    fn from(err: hex::FromHexError) -> CryptoError {
        CryptoError::HexError(err)
    }
}

impl From<std::string::FromUtf8Error> for CryptoError {
    fn from(err: std::string::FromUtf8Error) -> CryptoError {
        CryptoError::Utf8Error(err)
    }
}

impl From<chacha20poly1305::aead::Error> for CryptoError {
    fn from(err: chacha20poly1305::aead::Error) -> CryptoError {
        CryptoError::AeadError(err)
    }
}

pub fn encrypt_password(master_password: &str, password: &str) -> Result<String, CryptoError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let hashed_master_password = argon2.hash_password(master_password.as_bytes(), &salt)?;

    let binding = hashed_master_password.hash.ok_or(CryptoError::InvalidDataFormat)?;
    let key = Key::from_slice(binding.as_bytes());

    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = ChaCha20Poly1305::new(key);
    let ciphertext = cipher.encrypt(nonce, password.as_bytes())?;

    let mut encrypted_data = salt.as_str().to_string();
    encrypted_data.push(';');
    encrypted_data.push_str(&encode(nonce_bytes));
    encrypted_data.push(';');
    encrypted_data.push_str(&encode(ciphertext));

    let encoded_data = encode(encrypted_data);

    Ok(encoded_data)
}

pub fn decrypt_password(master_password: &str, encrypted_data: &str) -> Result<String, CryptoError> {
    let decoded_data = decode(encrypted_data)?;
    let decoded_data_str = String::from_utf8(decoded_data)?;

    let parts: Vec<&str> = decoded_data_str.split(';').collect();
    if parts.len() != 3 {
        return Err(CryptoError::InvalidDataFormat);
    }

    let salt = SaltString::from_b64(parts[0])?;
    let nonce_bytes = decode(parts[1])?;
    let ciphertext = decode(parts[2])?;

    let argon2 = Argon2::default();
    let hashed_master_password = argon2.hash_password(master_password.as_bytes(), &salt)?;

    let binding = hashed_master_password.hash.ok_or(CryptoError::InvalidDataFormat)?;
    let key = Key::from_slice(binding.as_bytes());

    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = ChaCha20Poly1305::new(key);
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;

    Ok(String::from_utf8(plaintext)?)
}

pub fn is_password_strong(password: &str) -> bool {
    let min_length = 12;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| c.is_ascii_punctuation());

    password.len() >= min_length && has_uppercase && has_lowercase && has_digit && has_special
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_encryption_decryption() {
        let master_password = "test_master_password";
        let password = "test_password";

        let encrypted_password = encrypt_password(master_password, password)
            .expect("Password encryption failed");
        let decrypted_password = decrypt_password(master_password, &encrypted_password)
            .expect("Password decryption failed");

        assert_eq!(password, decrypted_password);
    }

    #[test]
    fn test_incorrect_master_password() {
        let master_password = "test_master_password";
        let wrong_master_password = "wrong_password";
        let password = "test_password";

        let encrypted_password = encrypt_password(master_password, password)
            .expect("Password encryption failed");
        let decryption_result = decrypt_password(wrong_master_password, &encrypted_password);

        assert!(decryption_result.is_err());
    }

    #[test]
    fn test_password_policy() {
        assert!(is_password_strong("StrongP@ssword123"));
        assert!(!is_password_strong("weak"));
    }
}