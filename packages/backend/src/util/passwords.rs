use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use log::warn;
use passwords::{analyzer, scorer};
use rand::{rng, RngCore};
use thiserror::Error;

pub struct PasswordManager;
pub struct RandomPasswordHash {
    pub random_password: String,
    pub hash: String,
}

#[derive(Debug, Error, Clone)]
pub enum PasswordSecurityError {
    #[error("Password must be at least 12 characters")]
    TooShort,
    #[error("That password is too commonly used; please try something more unique")]
    TooCommon,
    #[error("Password was too weak; you need a score of at least 80 (current score {0}). Try adding numbers, symbols,
upper/lowercase characters, etc.")]
    LowScore(i64),
}

impl PasswordManager {
    pub fn check_security(password: &String) -> Result<(), PasswordSecurityError> {
        let analyzed_password = analyzer::analyze(password);

        if analyzed_password.length() < 12 {
            return Err(PasswordSecurityError::TooShort);
        }
        if analyzed_password.is_common() {
            return Err(PasswordSecurityError::TooCommon);
        }
        let score = scorer::score(&analyzed_password);
        if score < 80.0 {
            return Err(PasswordSecurityError::LowScore(score.floor() as i64));
        }
        Ok(())
    }
    pub fn hash(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash.to_string())
    }

    pub fn hash_random() -> Result<RandomPasswordHash, argon2::password_hash::Error> {
        let mut random_data = [0u8; 32];
        rng().fill_bytes(&mut random_data);
        let random_string = hex::encode(random_data).to_string();

        let hash = PasswordManager::hash(&random_string)?;

        Ok(RandomPasswordHash {
            random_password: random_string,
            hash,
        })
    }

    pub fn verify(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

        Ok(result.is_ok())
    }

    /// Time-wasting function to prevent time attacks
    pub fn dummy_verify(password: &str) {
        let res = Self::hash(password);
        if let Err(e) = res {
            warn!(
                "Failed to hash password during dummy_verify: {}",
                e.to_string()
            );
        }
    }
}
