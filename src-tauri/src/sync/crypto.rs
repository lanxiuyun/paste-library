//! Crypto Manager
//!
//! Encryption utilities using ChaCha20-Poly1305

use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use std::error::Error;

/// Crypto manager for encrypting/decrypting messages
pub struct CryptoManager {
    /// Cipher instance
    cipher: ChaCha20Poly1305,
}

impl CryptoManager {
    /// Create a new crypto manager with a generated key
    pub fn new() -> Result<Self, String> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = ChaCha20Poly1305::new(&key);

        Ok(Self { cipher })
    }

    /// Create with a specific key
    pub fn with_key(key: &[u8; 32]) -> Result<Self, String> {
        if key.len() != 32 {
            return Err("Invalid key length".to_string());
        }

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;

        Ok(Self { cipher })
    }

    /// Encrypt data
    pub fn encrypt(&self, plaintext: &[u8], nonce: &[u8; 12]) -> Result<Vec<u8>, String> {
        if nonce.len() != 12 {
            return Err("Invalid nonce length".to_string());
        }

        let nonce = Nonce::from_slice(nonce);
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {}", e))?;

        Ok(ciphertext)
    }

    /// Decrypt data
    pub fn decrypt(&self, ciphertext: &[u8], nonce: &[u8; 12]) -> Result<Vec<u8>, String> {
        if nonce.len() != 12 {
            return Err("Invalid nonce length".to_string());
        }

        let nonce = Nonce::from_slice(nonce);
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }
}

impl Default for CryptoManager {
    fn default() -> Self {
        Self::new().expect("Failed to create crypto manager")
    }
}
