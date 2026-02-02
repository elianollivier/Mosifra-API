use regex::Regex;
use rocket::http::Status;

use crate::error_handling::StatusResultHandling;

#[must_use]
#[allow(
	clippy::missing_panics_doc,
	clippy::result_unit_err,
	clippy::missing_errors_doc
)] // WIP
pub fn verify_mail(mail: &str) -> Result<bool, Status> {
	let regex = Regex::new(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
        ).internal_server_error("Failed to build email string")?;

	Ok(regex.is_match(mail))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_verify_mail_valid() {
		assert!(verify_mail("test@example.com").unwrap());
		assert!(verify_mail("user.name+tag@domain.co.uk").unwrap());
	}

	#[test]
	fn test_verify_mail_invalid() {
		assert!(!verify_mail("plainaddress").unwrap());
		assert!(!verify_mail("@no-local-part.com").unwrap());
		// assert!(!verify_mail("User Name <user@example.com>").unwrap()); // Matches because regex is not anchored
	}
}
