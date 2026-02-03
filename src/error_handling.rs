use rocket::http::Status;

pub trait StatusResultHandling<T, E: std::fmt::Debug> {
	fn internal_server_error<M: ToString>(self, message: M) -> Result<T, Status>;
	fn internal_server_error_no_message(self) -> Result<T, Status>;
}

impl<T, E: std::fmt::Debug> StatusResultHandling<T, E> for Result<T, E> {
	fn internal_server_error<M: ToString>(self, message: M) -> Result<T, Status> {
		match self {
			Ok(value) => Ok(value),
			Err(e) => {
				eprintln!("{} : {e:?}", message.to_string());
				Err(Status::InternalServerError)
			}
		}
	}

	fn internal_server_error_no_message(self) -> Result<T, Status> {
		match self {
			Ok(value) => Ok(value),
			Err(e) => {
				eprintln!("{e:?}");
				Err(Status::InternalServerError)
			}
		}
	}
}

pub trait StatusOptionHandling<T> {
	fn internal_server_error<M: ToString>(self, message: M) -> Result<T, Status>;
}

impl<T> StatusOptionHandling<T> for Option<T> {
	fn internal_server_error<M: ToString>(self, message: M) -> Result<T, Status> {
		match self {
			Some(value) => Ok(value),
			None => {
				eprintln!("{}", message.to_string());
				Err(Status::InternalServerError)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_result_internal_server_error_ok() {
		let result: Result<i32, &str> = Ok(42);
		let handled = result.internal_server_error("Test error");
		assert!(handled.is_ok());
		assert_eq!(handled.unwrap(), 42);
	}

	#[test]
	fn test_result_internal_server_error_err() {
		let result: Result<i32, &str> = Err("Something went wrong");
		let handled = result.internal_server_error("Test error");
		assert!(handled.is_err());
		assert_eq!(handled.unwrap_err(), Status::InternalServerError);
	}

	#[test]
	fn test_result_internal_server_error_no_message_ok() {
		let result: Result<String, &str> = Ok("success".to_string());
		let handled = result.internal_server_error_no_message();
		assert!(handled.is_ok());
		assert_eq!(handled.unwrap(), "success");
	}

	#[test]
	fn test_result_internal_server_error_no_message_err() {
		let result: Result<String, &str> = Err("failure");
		let handled = result.internal_server_error_no_message();
		assert!(handled.is_err());
		assert_eq!(handled.unwrap_err(), Status::InternalServerError);
	}

	#[test]
	fn test_option_internal_server_error_some() {
		let option: Option<i32> = Some(100);
		let handled = option.internal_server_error("No value found");
		assert!(handled.is_ok());
		assert_eq!(handled.unwrap(), 100);
	}

	#[test]
	fn test_option_internal_server_error_none() {
		let option: Option<i32> = None;
		let handled = option.internal_server_error("No value found");
		assert!(handled.is_err());
		assert_eq!(handled.unwrap_err(), Status::InternalServerError);
	}

	#[test]
	fn test_option_with_complex_type() {
		let option: Option<Vec<String>> = Some(vec!["test".to_string()]);
		let handled = option.internal_server_error("Vector not found");
		assert!(handled.is_ok());
		assert_eq!(handled.unwrap(), vec!["test".to_string()]);
	}
}
