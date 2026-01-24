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
