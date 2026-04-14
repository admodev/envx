//! Thin wrappers around the [`age`] passphrase encrypt/decrypt primitives.
//!
//! Both functions operate on raw byte slices so they are agnostic about file
//! format; the higher-level file I/O lives in [`crate::commands::encrypt`].

use age::secrecy::SecretString;
use anyhow::{Context, Result};
use std::io::{Read, Write};

/// Encrypt `data` with `passphrase` using age passphrase mode.
///
/// Returns the raw ciphertext bytes (armored age format).
///
/// # Errors
///
/// Propagates errors from the age encryptor if the underlying I/O fails.
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

/// Decrypt age-encrypted `data` with `passphrase`.
///
/// `data` must have been produced by [`encrypt`] (passphrase mode only —
/// public-key recipients are rejected).
///
/// # Errors
///
/// Returns an error if:
/// - the age header cannot be parsed,
/// - the file was encrypted with a public-key recipient instead of a passphrase,
/// - the passphrase is wrong, or
/// - the underlying I/O fails.
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
