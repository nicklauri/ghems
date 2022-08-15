use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use sha2::{Digest, Sha512};

#[allow(dead_code)]
pub fn sha512_with_salt(input: &str, salt: &str) -> [u8; 64] {
    let mut output = [0u8; 64];
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    hasher.update(b"$");
    hasher.update(salt.as_bytes());
    output.copy_from_slice(&hasher.finalize());

    output
}

#[allow(dead_code)]
pub fn sha512(input: &str) -> [u8; 64] {
    let mut output = [0u8; 64];
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    output.copy_from_slice(&hasher.finalize());

    output
}

// Argon2 password hashing helper.

pub fn generate_salt() -> String {
    SaltString::generate(rand::thread_rng()).to_string()
}

pub fn argon2(password: &str, salt: &str) -> anyhow::Result<String> {
    Ok(PasswordHash::generate(Argon2::default(), password, salt)
        .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
        .to_string())
}
