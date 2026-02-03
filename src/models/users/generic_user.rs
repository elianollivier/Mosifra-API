use std::any::Any;

use rocket::{http::Status, serde::json::Json};

use crate::{error_handling::StatusOptionHandling, redis, routes::auth::DisconnectResponse};

use super::{Company, Student, University, admin::Admin};

pub struct GenericUser {
	session_id: String,
	inner: Box<dyn Any + Send>,
}

impl GenericUser {
	pub fn new<T: 'static + Send>(value: T, session_id: String) -> Self {
		Self {
			inner: Box::new(value),
			session_id,
		}
	}

	pub fn is_university(&self) -> bool {
		self.inner.is::<University>()
	}

	pub fn to_university(&self) -> Result<&University, Status> {
		self.inner
			.downcast_ref::<University>()
			.internal_server_error("Cannot convert GenericUser to University")
	}

	pub fn is_student(&self) -> bool {
		self.inner.is::<Student>()
	}

	pub fn to_student(&self) -> Result<&Student, Status> {
		self.inner
			.downcast_ref::<Student>()
			.internal_server_error("Cannot convert GenericUser to Student")
	}

	pub fn is_company(&self) -> bool {
		self.inner.is::<Company>()
	}

	pub fn to_company(&self) -> Result<&Company, Status> {
		self.inner
			.downcast_ref::<Company>()
			.internal_server_error("Cannot convert GenericUser to Company")
	}

	pub fn is_admin(&self) -> bool {
		self.inner.is::<Admin>()
	}

	pub fn to_admin(&self) -> Result<&Admin, Status> {
		self.inner
			.downcast_ref::<Admin>()
			.internal_server_error("Cannot convert GenericUser to Company")
	}

	pub fn logout(&self) -> Result<Json<DisconnectResponse>, Status> {
		redis::invalidate_session(&self.session_id)?;
		Ok(Json(DisconnectResponse { success: true }))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_generic_user_admin_type_check() {
		let admin = Admin::default();
		let generic_user = GenericUser::new(admin, "test_session".to_string());
		
		assert!(generic_user.is_admin());
		assert!(!generic_user.is_student());
		assert!(!generic_user.is_university());
		assert!(!generic_user.is_company());
	}

	#[test]
	fn test_generic_user_admin_conversion() {
		let admin = Admin::default();
		let generic_user = GenericUser::new(admin, "test_session".to_string());
		
		assert!(generic_user.to_admin().is_ok());
		assert!(generic_user.to_student().is_err());
		assert!(generic_user.to_university().is_err());
		assert!(generic_user.to_company().is_err());
	}

	#[test]
	fn test_generic_user_student_type_check() {
		let student = Student {
			id: "test_id".to_string(),
			login: "test_login".to_string(),
			password: "test_password".to_string(),
			mail: "test@example.com".to_string(),
			first_name: "Test".to_string(),
			last_name: "User".to_string(),
		};
		let generic_user = GenericUser::new(student, "test_session".to_string());
		
		assert!(generic_user.is_student());
		assert!(!generic_user.is_admin());
		assert!(!generic_user.is_university());
		assert!(!generic_user.is_company());
	}

	#[test]
	fn test_generic_user_student_conversion() {
		let student = Student {
			id: "test_id".to_string(),
			login: "test_login".to_string(),
			password: "test_password".to_string(),
			mail: "test@example.com".to_string(),
			first_name: "Test".to_string(),
			last_name: "User".to_string(),
		};
		let generic_user = GenericUser::new(student, "test_session".to_string());
		
		assert!(generic_user.to_student().is_ok());
		assert!(generic_user.to_admin().is_err());
		assert!(generic_user.to_university().is_err());
		assert!(generic_user.to_company().is_err());
	}

	#[test]
	fn test_generic_user_student_data_integrity() {
		let student = Student {
			id: "test_id".to_string(),
			login: "test_login".to_string(),
			password: "test_password".to_string(),
			mail: "test@example.com".to_string(),
			first_name: "Test".to_string(),
			last_name: "User".to_string(),
		};
		let generic_user = GenericUser::new(student, "test_session".to_string());
		
		let retrieved_student = generic_user.to_student().unwrap();
		assert_eq!(retrieved_student.id, "test_id");
		assert_eq!(retrieved_student.login, "test_login");
		assert_eq!(retrieved_student.mail, "test@example.com");
	}
}
