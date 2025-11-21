mod hashing_error;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use crate::model::values::password::Password;
use crate::model::values::password_hash::PasswordHash;
use crate::utils::hasher::hashing_error::HashingError;

#[derive(Clone)]
pub struct Hasher {
    pepper: String,
}

impl Hasher {
    pub fn new(pepper: String) -> Self {
        Hasher { pepper }
    }

    fn pepper_password(&self, password: &str) -> String {
        format!("{}{}", password, self.pepper)
    }

    pub fn hash_password(&self, password: &Password) -> Result<PasswordHash, HashingError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let peppered_password = self.pepper_password(&password);

        let hash = argon2
            .hash_password(peppered_password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| HashingError::HashingError(e.to_string()))?;

        Ok(hash.into())

    }

    pub fn verify_password(&self, password: &Password, hash: &PasswordHash) -> Result<bool, HashingError> {
        let parsed_hash = argon2::PasswordHash::new(hash)
            .map_err(|e| HashingError::VerificationError(e.to_string()))?;

        let argon2 = Argon2::default();

        let peppered_password = self.pepper_password(password);

        match argon2.verify_password(peppered_password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::hasher::Hasher;

    #[test]
    fn test_verify_password_correct() {
        let hasher = Hasher::new("test_pepper".to_string());
        let password = "my_secret_password".try_into().unwrap();
        let hash = hasher
            .hash_password(&password)
            .expect("Failed to hash password");

        let result = hasher
            .verify_password(&password, &hash)
            .expect("Failed to verify password");
        assert!(result, "Password verification should succeed");
    }

    #[test]
    fn test_verify_password_incorrect() {
        let hasher = Hasher::new("test_pepper".to_string());
        let password = "correct_password".try_into().unwrap();
        let wrong_password = "wrong_password".try_into().unwrap();
        let hash = hasher
            .hash_password(&password)
            .expect("Failed to hash password");

        let result = hasher
            .verify_password(&wrong_password, &hash)
            .expect("Failed to verify password");
        assert!(
            !result,
            "Password verification should fail for incorrect password"
        );
    }

    #[test]
    fn test_hash_produces_different_hashes() {
        let hasher = Hasher::new("test_pepper".to_string());
        let password = "same_password".try_into().unwrap();
        let hash1 = hasher
            .hash_password(&password)
            .expect("Failed to hash password");
        let hash2 = hasher
            .hash_password(&password)
            .expect("Failed to hash password");

        // Same password should produce different hashes due to random salt
        assert_ne!(
            hash1, hash2,
            "Same password should produce different hashes"
        );

        // But both should verify correctly
        assert!(hasher.verify_password(&password, &hash1).unwrap());
        assert!(hasher.verify_password(&password, &hash2).unwrap());
    }

    #[test]
    fn test_different_pepper_fails_verification() {
        let hasher1 = Hasher::new("pepper1".to_string());
        let hasher2 = Hasher::new("pepper2".to_string());
        let password = "test_password".try_into().unwrap();

        let hash = hasher1
            .hash_password(&password)
            .expect("Failed to hash password");

        // Verifying with different pepper should fail
        let result = hasher2
            .verify_password(&password, &hash)
            .expect("Failed to verify password");
        assert!(
            !result,
            "Password verification should fail with different pepper"
        );
    }
}
