use anyhow::{bail, Context, Result};
use owo_colors::OwoColorize;
use std::path::Path;

pub fn run_encrypt(file: &Path) -> Result<()> {
    let pass1 = rpassword::prompt_password("Passphrase: ").context("failed to read passphrase")?;
    let pass2 =
        rpassword::prompt_password("Confirm passphrase: ").context("failed to read passphrase")?;

    if pass1 != pass2 {
        bail!("passphrases do not match");
    }

    let data = std::fs::read(file).with_context(|| format!("failed to read {}", file.display()))?;

    let encrypted = crate::crypto::encrypt(&data, &pass1)?;

    let out_path = {
        let mut p = file.as_os_str().to_owned();
        p.push(".age");
        std::path::PathBuf::from(p)
    };

    std::fs::write(&out_path, &encrypted)
        .with_context(|| format!("failed to write {}", out_path.display()))?;

    println!("{}", format!("Encrypted → {}", out_path.display()).green());

    Ok(())
}

pub fn run_decrypt(file: &Path) -> Result<()> {
    if file.extension().and_then(|e| e.to_str()) != Some("age") {
        bail!("file does not have .age extension: {}", file.display());
    }

    let pass = rpassword::prompt_password("Passphrase: ").context("failed to read passphrase")?;

    let data = std::fs::read(file).with_context(|| format!("failed to read {}", file.display()))?;

    let decrypted = crate::crypto::decrypt(&data, &pass)?;

    // Strip the .age suffix to get the output path
    let out_path = file.with_extension("");

    std::fs::write(&out_path, &decrypted)
        .with_context(|| format!("failed to write {}", out_path.display()))?;

    println!("{}", format!("Decrypted → {}", out_path.display()).green());

    Ok(())
}
