use crate::cli::cli::EncodingFormat;
use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use bcrypt::hash_with_salt;
use rand::{Rng, rng};
use sha2::{Digest, Sha256, Sha512};

pub fn encode_password(password: &str, format: &EncodingFormat) -> Result<String> {
    match format {
        EncodingFormat::None => Ok(password.to_string()),

        EncodingFormat::Base64 => Ok(STANDARD.encode(password.as_bytes())),

        EncodingFormat::Url => Ok(urlencoding::encode(password).to_string()),

        EncodingFormat::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            let result = hasher.finalize();
            Ok(hex::encode(result))
        }

        EncodingFormat::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(password.as_bytes());
            let result = hasher.finalize();
            Ok(hex::encode(result))
        }

        EncodingFormat::Htpasswd => {
            let hashed = encode_htpasswd_bcrypt(password, 10)?;
            Ok(format!("{}", hashed))
        }
    }
}

fn encode_htpasswd_bcrypt(password: &str, cost: u32) -> Result<String> {
    let mut salt = [0u8; 16];
    rng().fill(&mut salt);

    hash_with_salt(password, cost, salt)
        .map(|hash| hash.to_string())
        .context("Failed to hash password")
}
