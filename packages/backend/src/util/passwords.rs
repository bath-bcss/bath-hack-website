use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use log::warn;
use rand::{rngs::OsRng, RngCore};

pub struct PasswordManager;
pub struct RandomPasswordHash {
    pub random_password: String,
    pub hash: String,
}

impl PasswordManager {
    pub fn hash(password: &String) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash.to_string())
    }

    pub fn hash_random() -> Result<RandomPasswordHash, argon2::password_hash::Error> {
        let mut random_data = [0u8; 32];
        OsRng.fill_bytes(&mut random_data);
        let random_string = hex::encode(&random_data).to_string();

        let hash = PasswordManager::hash(&random_string)?;

        Ok(RandomPasswordHash {
            random_password: random_string,
            hash,
        })
    }

    pub fn verify(password: &String, hash: &String) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

        Ok(result.is_ok())
    }

    /// Time-wasting function to prevent time attacks
    pub fn dummy_verify(password: &String) {
        let res = Self::hash(password);
        if let Err(e) = res {
            warn!("Failed to hash password during dummy_verify: {}", e.to_string());
        }
    }
}
