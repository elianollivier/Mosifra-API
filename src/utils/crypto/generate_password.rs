use passwords::PasswordGenerator;
use rocket::http::Status;

use crate::error_handling::StatusResultHandling;

#[allow(clippy::missing_errors_doc)]
pub fn generate_password() -> Result<String, Status> {
	PasswordGenerator::new()
		.length(8)
		.numbers(true)
		.lowercase_letters(true)
		.uppercase_letters(true)
		.symbols(true)
		.spaces(false)
		.exclude_similar_characters(true)
		.strict(true)
		.generate_one()
		.internal_server_error("Error while generating password")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_generate_password_length() {
		let pwd = generate_password().unwrap();
		assert_eq!(pwd.len(), 8);
	}

	#[test]
	fn test_generate_password_success() {
		let pwd = generate_password();
		assert!(pwd.is_ok());
		let pwd = pwd.unwrap();
		assert!(!pwd.is_empty());
	}
}
