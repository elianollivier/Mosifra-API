use std::str::FromStr;

use rocket::{http::Status, serde::json::Json};
use tracing::{info, warn, instrument};

use crate::{
	models::{
		auth::UserType,
		users::{Company, Student, University},
	},
	postgres::Db,
	redis::get_transactionid,
	utils::mail::send_2fa_mail,
};

use super::domain::{LoginPayload, LoginResponse};

#[post("/auth/login", data = "<login_payload>")]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::missing_errors_doc)]
#[instrument(skip(login_payload), fields(user_type = %login_payload.user_type))]
pub async fn login(login_payload: Json<LoginPayload>) -> Result<Json<LoginResponse>, Status> {
	let login = login_payload.into_inner();
	info!(login = %login.login, "Login attempt received");
	
	let user_type = UserType::from_str(&login.user_type)?;

	match user_type {
		UserType::Admin => todo!(),
		UserType::University => login_university(login).await,
		UserType::Student => login_student(login).await,
		UserType::Company => login_company(login).await,
	}
}

pub async fn login_university(login: LoginPayload) -> Result<Json<LoginResponse>, Status> {
	let university = University::login(&login.login, &login.password).await?;

	match university {
		Some(university) => {
			info!(university_id = %university.id, "University login successful");
			set_transaction_id(&university.mail, &university.id, login.remember_me)
		}
		None => {
			warn!(login = %login.login, "University login failed - invalid credentials");
			Ok(Json(LoginResponse {
				valid: false,
				transaction_id: None,
				remember_me: None,
			}))
		}
	}
}

pub async fn login_company(login: LoginPayload) -> Result<Json<LoginResponse>, Status> {
	let company = Company::login(&login.login, &login.password).await?;

	match company {
		Some(company) => {
			info!(company_id = %company.id, "Company login successful");
			set_transaction_id(&company.mail, &company.id, login.remember_me)
		}
		None => {
			warn!(login = %login.login, "Company login failed - invalid credentials");
			Ok(Json(LoginResponse {
				valid: false,
				transaction_id: None,
				remember_me: None,
			}))
		}
	}
}

pub async fn login_student(login: LoginPayload) -> Result<Json<LoginResponse>, Status> {
	let student = Student::login(&login.login, &login.password).await?;

	match student {
		Some(student) => {
			info!(student_id = %student.id, "Student login successful");
			set_transaction_id(&student.mail, &student.id, login.remember_me)
		}
		None => {
			warn!(login = %login.login, "Student login failed - invalid credentials");
			Ok(Json(LoginResponse {
				valid: false,
				transaction_id: None,
				remember_me: None,
			}))
		}
	}
}

pub fn set_transaction_id(
	mail: &str,
	id: &str,
	remember_me: bool,
) -> Result<Json<LoginResponse>, Status> {
	let code = send_2fa_mail(mail)?;
	let transaction_id = get_transactionid(id, code)?;
	Ok(Json(LoginResponse {
		valid: true,
		transaction_id: Some(transaction_id),
		remember_me: Some(remember_me),
	}))
}
