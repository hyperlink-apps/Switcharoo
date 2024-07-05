use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub struct Password {}

impl Password {
    pub fn hash(password: &str) -> Result<String, String> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let hash = match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Error hashing password: {}", e.to_string());
                return Err(String::from("Error hashing password."));
            }
        }
        .to_string();

        Ok(hash)
    }

    pub fn validate(password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn check_strength(password: &str) -> Option<String> {
        if password.len() < 8 {
            return Some("Password should be at least 8 characters long.".to_string());
        }

        if !password.chars().any(|c| c.is_numeric()) {
            return Some("Password should contain at least one number.".to_string());
        }

        if !password.chars().any(|c| c.is_alphabetic()) {
            return Some("Password should contain at least one letter.".to_string());
        }

        if !password.chars().any(|c| c.is_lowercase()) {
            return Some("Password should contain at least one lowercase letter.".to_string());
        }

        if !password.chars().any(|c| c.is_uppercase()) {
            return Some("Password should contain at least one uppercase letter.".to_string());
        }

        None
    }
}
