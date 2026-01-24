use argon2::{
	Argon2, PasswordHasher,
	password_hash::{SaltString, rand_core::OsRng},
};
use rocket::http::Status;

use crate::error_handling::StatusResultHandling;

#[allow(clippy::missing_errors_doc)]
pub fn hash_password(password: &str) -> Result<String, Status> {
	let bytes_password = password.as_bytes();
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();

	let password_hash = argon2
		.hash_password(bytes_password, &salt)
		.internal_server_error("Error while tryinng to hash password")?
		.to_string();

	Ok(password_hash)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hash_password_returns_hash() {
		let result = hash_password("testpassword123");
		assert!(result.is_ok());
		let hash = result.unwrap();
		assert!(hash.starts_with("$argon2"));
	}

	#[test]
	fn test_hash_password_different_each_time() {
		let hash1 = hash_password("samepassword").unwrap();
		let hash2 = hash_password("samepassword").unwrap();
		assert_ne!(hash1, hash2);
	}
}
