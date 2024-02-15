use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
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
}
