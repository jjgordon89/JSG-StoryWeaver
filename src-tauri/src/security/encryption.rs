//! Data encryption for sensitive information
//!
//! This module provides functionality for encrypting and decrypting sensitive data
//! using strong encryption algorithms.

use crate::error::StoryWeaverError;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::fs;
use std::path::PathBuf;
use tauri::api::path;

/// Encryption manager for handling sensitive data
#[derive(Debug)]
pub struct EncryptionManager {
    cipher: Arc<RwLock<Aes256Gcm>>,
}

impl EncryptionManager {
    /// Create a new encryption manager
    pub async fn new() -> Result<Self, StoryWeaverError> {
        let key_path = get_key_path()?;
        let key = load_or_create_key(&key_path)?;
        
        let cipher = Aes256Gcm::new(&key);
        
        Ok(Self {
            cipher: Arc::new(RwLock::new(cipher)),
        })
    }

    /// Encrypt data
    pub async fn encrypt(&self, data: &str) -> Result<String, StoryWeaverError> {
        let cipher = self.cipher.read().await;
        
        // Generate a random nonce
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        // Encrypt the data
        let ciphertext = cipher
            .encrypt(&nonce, data.as_bytes())
            .map_err(|e| StoryWeaverError::SecurityError(format!("Encryption failed: {}", e)))?;
        
        // Combine nonce and ciphertext and encode as base64
        let mut combined = nonce.to_vec();
        combined.extend_from_slice(&ciphertext);
        
        Ok(BASE64.encode(combined))
    }

    /// Decrypt data
    pub async fn decrypt(&self, encrypted_data: &str) -> Result<String, StoryWeaverError> {
        let cipher = self.cipher.read().await;
        
        // Decode from base64
        let combined = BASE64.decode(encrypted_data)
            .map_err(|e| StoryWeaverError::SecurityError(format!("Base64 decoding failed: {}", e)))?;
        
        // Split into nonce and ciphertext
        if combined.len() < 12 {
            return Err(StoryWeaverError::SecurityError("Invalid encrypted data".to_string()));
        }
        
        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        // Decrypt the data
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| StoryWeaverError::SecurityError(format!("Decryption failed: {}", e)))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| StoryWeaverError::SecurityError(format!("UTF-8 decoding failed: {}", e)))
    }
}

/// Get the path to the encryption key file
fn get_key_path() -> Result<PathBuf, StoryWeaverError> {
    let app_data_dir = path::app_data_dir(&tauri::Config::default())
        .ok_or_else(|| StoryWeaverError::SecurityError("Failed to get app data directory".to_string()))?;
    
    let key_dir = app_data_dir.join("keys");
    fs::create_dir_all(&key_dir)
        .map_err(|e| StoryWeaverError::SecurityError(format!("Failed to create key directory: {}", e)))?;
    
    Ok(key_dir.join("encryption_key.bin"))
}

/// Load an existing key or create a new one
fn load_or_create_key(key_path: &PathBuf) -> Result<Key<Aes256Gcm>, StoryWeaverError> {
    if key_path.exists() {
        // Load existing key
        let key_bytes = fs::read(key_path)
            .map_err(|e| StoryWeaverError::SecurityError(format!("Failed to read encryption key: {}", e)))?;
        
        if key_bytes.len() != 32 {
            return Err(StoryWeaverError::SecurityError("Invalid encryption key length".to_string()));
        }
        
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes).clone();
        Ok(key)
    } else {
        // Generate a new key
        let key = Aes256Gcm::generate_key(&mut OsRng);
        
        // Save the key
        fs::write(key_path, key.as_slice())
            .map_err(|e| StoryWeaverError::SecurityError(format!("Failed to save encryption key: {}", e)))?;
        
        Ok(key)
    }
}

/// Global instance of the encryption manager
static mut ENCRYPTION_MANAGER: Option<Arc<EncryptionManager>> = None;

/// Initialize the encryption manager
pub async fn init() -> Result<(), StoryWeaverError> {
    let manager = EncryptionManager::new().await?;
    
    unsafe {
        ENCRYPTION_MANAGER = Some(Arc::new(manager));
    }
    
    Ok(())
}

/// Get the global encryption manager instance
pub fn get_encryption_manager() -> Result<Arc<EncryptionManager>, StoryWeaverError> {
    unsafe {
        match &ENCRYPTION_MANAGER {
            Some(manager) => Ok(manager.clone()),
            None => Err(StoryWeaverError::SecurityError("Encryption manager not initialized".to_string())),
        }
    }
}

/// Helper function to encrypt a string
pub async fn encrypt_string(data: &str) -> Result<String, StoryWeaverError> {
    let manager = get_encryption_manager()?;
    manager.encrypt(data).await
}

/// Helper function to decrypt a string
pub async fn decrypt_string(encrypted_data: &str) -> Result<String, StoryWeaverError> {
    let manager = get_encryption_manager()?;
    manager.decrypt(encrypted_data).await
}
