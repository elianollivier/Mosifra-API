use std::{fmt::Display, str::FromStr};

use rocket::http::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
	Admin,
	University,
	Student,
	Company,
}

impl Display for UserType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Admin => write!(f, "admin"),
			Self::University => write!(f, "university"),
			Self::Student => write!(f, "student"),
			Self::Company => write!(f, "company"),
		}
	}
}

impl FromStr for UserType {
	type Err = Status;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		match value {
			"admin" => Ok(Self::Admin),
			"university" => Ok(Self::University),
			"student" => Ok(Self::Student),
			"company" => Ok(Self::Company),
			_ => Err(Status::InternalServerError),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_user_type_display() {
		assert_eq!(UserType::Admin.to_string(), "admin");
		assert_eq!(UserType::University.to_string(), "university");
		assert_eq!(UserType::Student.to_string(), "student");
		assert_eq!(UserType::Company.to_string(), "company");
	}

	#[test]
	fn test_user_type_from_str_valid() {
		assert_eq!(UserType::from_str("admin").unwrap(), UserType::Admin);
		assert_eq!(UserType::from_str("university").unwrap(), UserType::University);
		assert_eq!(UserType::from_str("student").unwrap(), UserType::Student);
		assert_eq!(UserType::from_str("company").unwrap(), UserType::Company);
	}

	#[test]
	fn test_user_type_from_str_invalid() {
		assert!(UserType::from_str("invalid").is_err());
		assert!(UserType::from_str("ADMIN").is_err());
		assert!(UserType::from_str("").is_err());
		assert!(UserType::from_str("teacher").is_err());
	}

	#[test]
	fn test_user_type_roundtrip() {
		let types = vec![
			UserType::Admin,
			UserType::University,
			UserType::Student,
			UserType::Company,
		];
		
		for user_type in types {
			let string = user_type.to_string();
			let parsed = UserType::from_str(&string).unwrap();
			assert_eq!(parsed, user_type);
		}
	}
}
