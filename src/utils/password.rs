use super::crypto;

/// Hash `password` and return `(password_hash, salt)`.
pub async fn hash(password: &str) -> anyhow::Result<(String, String)> {
    let password = password.to_owned();

    let result = tokio::task::spawn_blocking(move || -> anyhow::Result<(String, String)> {
        let salt = crypto::generate_salt();

        let password_hash = crypto::argon2(&password, &salt)?;

        Ok((password_hash, salt))
    })
    .await??;

    Ok(result)
}

pub async fn verify(password: &str, password_hash: &str, salt: &str) -> anyhow::Result<bool> {
    let password = password.to_owned();
    let password_hash = password_hash.to_owned();
    let salt = salt.to_owned();

    let result = tokio::task::spawn_blocking(move || -> anyhow::Result<bool> {
        let password_hashed = crypto::argon2(&password, &salt)?;

        let result = password_hashed == password_hash;

        Ok(result)
    })
    .await??;

    Ok(result)
}
