use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rocket::http::Status;

use crate::error_handling::StatusResultHandling;

#[allow(clippy::missing_errors_doc)]
pub fn verify_password(pwd_to_check: &str, stored_hash: &str) -> Result<bool, Status> {
	let parsed_hash =
		PasswordHash::new(stored_hash).internal_server_error("Erreur parsing hash")?;
	let is_correct = Argon2::default()
		.verify_password(pwd_to_check.as_bytes(), &parsed_hash)
		.map(|()| true);

	Ok(is_correct.is_ok())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::crypto::hash_password;

	#[test]
	fn test_verify_correct_password() {
		let hash = hash_password("mypassword").unwrap();
		let result = verify_password("mypassword", &hash);
		assert!(result.is_ok());
		assert!(result.unwrap());
	}

	#[test]
	fn test_verify_wrong_password() {
		let hash = hash_password("correctpassword").unwrap();
		let result = verify_password("wrongpassword", &hash);
		assert!(result.is_ok());
		assert!(!result.unwrap());
	}
}
