use rocket::http::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CourseType {
	Info,
}

pub const INFO_DB: i32 = 1;

impl CourseType {
	#[must_use]
	pub const fn to_sql(&self) -> i32 {
		match self {
			Self::Info => INFO_DB,
		}
	}

	pub const fn from_sql(course_type_id: i32) -> Result<Self, Status> {
		match course_type_id {
			INFO_DB => Ok(Self::Info),
			_ => Err(Status::InternalServerError),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_course_type_to_sql() {
		assert_eq!(CourseType::Info.to_sql(), 1);
	}

	#[test]
	fn test_course_type_from_sql_valid() {
		let result = CourseType::from_sql(1);
		assert!(result.is_ok());
		assert_eq!(result.unwrap(), CourseType::Info);
	}

	#[test]
	fn test_course_type_from_sql_invalid() {
		let result = CourseType::from_sql(999);
		assert!(result.is_err());
	}
}
