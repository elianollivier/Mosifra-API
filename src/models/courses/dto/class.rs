use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::courses::{Class, CourseType};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClassDto {
	pub id: String,
	pub name: String,
	pub course_type: CourseType,
	pub date_internship_start: NaiveDate,
	pub date_internship_end: NaiveDate,
	pub maximum_internship_length: i32,
	pub minimum_internship_length: i32,
}

impl ClassDto {
	pub fn from_vec(class_list: Vec<Class>) -> Vec<Self> {
		let mut res = vec![];
		for class in class_list {
			res.push(Self {
				id: class.id,
				name: class.name,
				course_type: class.course_type,
				date_internship_start: class.date_internship_start,
				date_internship_end: class.date_internship_end,
				maximum_internship_length: class.maximum_internship_length,
				minimum_internship_length: class.minimum_internship_length,
			});
		}

		res
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_class_dto_from_vec_empty() {
		let classes: Vec<Class> = vec![];
		let dtos = ClassDto::from_vec(classes);
		assert!(dtos.is_empty());
	}

	#[test]
	fn test_class_dto_from_vec_single() {
		let class = Class {
			id: "test_id".to_string(),
			name: "Test Class".to_string(),
			course_type: CourseType::Info,
			date_internship_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
			date_internship_end: NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
			maximum_internship_length: 180,
			minimum_internship_length: 90,
			university_id: "uni_123".to_string(),
		};

		let dtos = ClassDto::from_vec(vec![class]);
		assert_eq!(dtos.len(), 1);
		assert_eq!(dtos[0].id, "test_id");
		assert_eq!(dtos[0].name, "Test Class");
	}

	#[test]
	fn test_class_dto_from_vec_multiple() {
		let class1 = Class {
			id: "id1".to_string(),
			name: "Class 1".to_string(),
			course_type: CourseType::Info,
			date_internship_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
			date_internship_end: NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
			maximum_internship_length: 180,
			minimum_internship_length: 90,
			university_id: "uni_123".to_string(),
		};

		let class2 = Class {
			id: "id2".to_string(),
			name: "Class 2".to_string(),
			course_type: CourseType::Info,
			date_internship_start: NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
			date_internship_end: NaiveDate::from_ymd_opt(2024, 7, 31).unwrap(),
			maximum_internship_length: 200,
			minimum_internship_length: 100,
			university_id: "uni_456".to_string(),
		};

		let dtos = ClassDto::from_vec(vec![class1, class2]);
		assert_eq!(dtos.len(), 2);
		assert_eq!(dtos[0].id, "id1");
		assert_eq!(dtos[1].id, "id2");
	}

	#[test]
	fn test_class_dto_data_integrity() {
		let class = Class {
			id: "test_id".to_string(),
			name: "Test Class".to_string(),
			course_type: CourseType::Info,
			date_internship_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
			date_internship_end: NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
			maximum_internship_length: 180,
			minimum_internship_length: 90,
			university_id: "uni_123".to_string(),
		};

		let dtos = ClassDto::from_vec(vec![class]);
		assert_eq!(dtos[0].maximum_internship_length, 180);
		assert_eq!(dtos[0].minimum_internship_length, 90);
		assert_eq!(dtos[0].date_internship_start, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
		assert_eq!(dtos[0].date_internship_end, NaiveDate::from_ymd_opt(2024, 6, 30).unwrap());
	}
}
