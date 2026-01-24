use std::{collections::HashSet, env, process::exit};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::{
	Request,
	http::Status,
	request::{FromRequest, Outcome},
};
use serde::{Deserialize, Serialize};

use crate::{
	error_handling::StatusResultHandling,
	models::users::{Company, GenericUser, Student, University, admin::Admin},
	redis::{self, session_exist},
};

use super::UserType;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	session_id: String,
	user_type: UserType,
}

#[derive(Debug)]
pub struct AuthGuard {
	pub session_id: String,
	pub user_type: UserType,
}

impl AuthGuard {
	fn from_raw_jwt(raw_jwt: &str) -> Result<Self, String> {
		let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
			|| {
				eprintln!("JWT Secret must be in .env");
				exit(1)
			},
			|secret| secret,
		);

		let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
		validation.required_spec_claims = HashSet::new();
		validation.validate_exp = false;

		let token = decode::<Claims>(
			&raw_jwt,
			&DecodingKey::from_secret(jwt_secret.as_bytes()),
			&validation,
		)
		.map_err(|e| format!("JWT is not valid: {e}"))?;

		Ok(Self {
			session_id: token.claims.session_id,
			user_type: token.claims.user_type,
		})
	}

	pub fn new_raw_jwt_from_data(
		session_id: String,
		user_type: UserType,
	) -> Result<Option<String>, Status> {
		let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
			|| {
				eprintln!("JWT Secret must be in .env");
				exit(1)
			},
			|secret| secret,
		);

		if !session_exist(&session_id)? {
			return Ok(None);
		}

		let claims = Claims {
			session_id,
			user_type,
		};

		let jwt = encode(
			&Header::default(),
			&claims,
			&EncodingKey::from_secret(jwt_secret.as_bytes()),
		)
		.internal_server_error("Failed to create JWT token")?;

		Ok(Some(jwt))
	}

	pub async fn get_generic_user(&self) -> Result<GenericUser, Status> {
		match self.user_type {
			UserType::Admin => Ok(GenericUser::new(Admin::default(), String::new())),
			UserType::University => Ok(GenericUser::new(
				University::from_id(self.get_user_id()?).await?,
				self.session_id.clone(),
			)),
			UserType::Student => Ok(GenericUser::new(
				Student::from_id(self.get_user_id()?).await?,
				self.session_id.clone(),
			)),
			UserType::Company => Ok(GenericUser::new(
				Company::from_id(self.get_user_id()?).await?,
				self.session_id.clone(),
			)),
		}
	}

	pub fn get_user_id(&self) -> Result<String, Status> {
		redis::get_user_id_from_session_id(self.session_id.clone())
	}
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
	type Error = String;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let auth_header = request.headers().get_one("Authorization");

		let header = match auth_header {
			Some(h) if h.starts_with("Bearer ") => h,
			_ => return Outcome::Error((Status::Unauthorized, "Authorization header missing".to_string())),
		};

		let jwt = header.trim_start_matches("Bearer ");

		if !validate_jwt(jwt) {
			return Outcome::Error((Status::Unauthorized, "Invalid Token".to_string()));
		}

		let auth_guard = match Self::from_raw_jwt(jwt) {
			Ok(guard) => guard,
			Err(e) => return Outcome::Error((
				Status::InternalServerError,
				format!("Error while getting the jwt information: {e}")
			)),
		};

		if auth_guard.user_type == UserType::Admin {
			return Outcome::Success(auth_guard);
		}

		match session_exist(&auth_guard.session_id) {
			Ok(true) => Outcome::Success(auth_guard),
			Ok(false) => Outcome::Error((Status::Unauthorized, "Session expired".to_string())),
			Err(e) => Outcome::Error((e, "Error while checking session".to_string())),
		}
	}
}

fn validate_jwt(jwt: &str) -> bool {
	let jwt_secret = env::var("JWT_SECRET").ok().map_or_else(
		|| {
			eprintln!("JWT Secret must be in .env");
			exit(1)
		},
		|secret| secret,
	);

	let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
	validation.required_spec_claims = HashSet::new();
	validation.validate_exp = false;

	let token = decode::<Claims>(
		&jwt,
		&DecodingKey::from_secret(jwt_secret.as_bytes()),
		&validation,
	);

	match token {
		Ok(_) => true,
		Err(e) => {
			println!("{e}");
			false
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_invalid_jwt_format() {
		std::env::set_var("JWT_SECRET", "testsecret123456");
		assert!(!validate_jwt("not.a.valid.jwt"));
		assert!(!validate_jwt(""));
		assert!(!validate_jwt("random_string"));
	}

	#[test]
	fn test_jwt_wrong_signature() {
		std::env::set_var("JWT_SECRET", "testsecret123456");
		let fake_jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.wrongsignature";
		assert!(!validate_jwt(fake_jwt));
	}
}
