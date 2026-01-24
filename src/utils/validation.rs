use regex::Regex;
use rocket::http::Status;

const MIN_PASSWORD_LENGTH: usize = 8;

pub fn validate_password(password: &str) -> Result<(), Status> {
	if password.len() < MIN_PASSWORD_LENGTH {
		return Err(Status::BadRequest);
	}
	Ok(())
}

pub fn validate_email(email: &str) -> Result<(), Status> {
	let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
		.map_err(|_| Status::InternalServerError)?;

	if !email_regex.is_match(email) {
		return Err(Status::BadRequest);
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_password_too_short() {
		assert!(validate_password("1234567").is_err());
	}

	#[test]
	fn test_password_valid() {
		assert!(validate_password("12345678").is_ok());
		assert!(validate_password("verylongpassword").is_ok());
	}

	#[test]
	fn test_email_valid() {
		assert!(validate_email("test@example.com").is_ok());
		assert!(validate_email("user.name@domain.fr").is_ok());
	}

	#[test]
	fn test_email_invalid() {
		assert!(validate_email("invalid").is_err());
		assert!(validate_email("no@domain").is_err());
		assert!(validate_email("@nodomain.com").is_err());
	}
}
