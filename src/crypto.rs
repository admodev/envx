use age::secrecy::SecretString;
use anyhow::{Context, Result};
use std::io::{Read, Write};

/// Encrypts plaintext bytes with a passphrase using age.
pub fn encrypt(data: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    let secret = SecretString::from(passphrase.to_string());
    let encryptor = age::Encryptor::with_user_passphrase(secret);

    let mut output = Vec::new();
    let mut writer = encryptor
        .wrap_output(&mut output)
        .context("failed to create age encryptor")?;

    writer
        .write_all(data)
        .context("failed to write plaintext")?;
    writer.finish().context("failed to finalize encryption")?;

    Ok(output)
}

/// Decrypts age-encrypted bytes with a passphrase.
pub fn decrypt(data: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    let secret = SecretString::from(passphrase.to_string());

    let decryptor = match age::Decryptor::new(data).context("failed to parse age header")? {
        age::Decryptor::Passphrase(d) => d,
        _ => anyhow::bail!("expected passphrase-encrypted file"),
    };

    let mut reader = decryptor
        .decrypt(&secret, None)
        .context("decryption failed — wrong passphrase?")?;

    let mut output = Vec::new();
    reader
        .read_to_end(&mut output)
        .context("failed to read decrypted data")?;

    Ok(output)
}
