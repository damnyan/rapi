use actix_web::{get, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct ExampleResource {
	pub id: i32,
	#[validate(length(min = 1, message = "Name must not be empty"))]
	pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct SomeObject {
	#[validate(length(min = 1, message = "Value must not be empty"))]
	pub value: String,
}

#[utoipa::path(
	get,
	path = "/example",
	responses(
		(status = 200, description = "Example response", body = ExampleResource)
	)
)]
#[get("/example")]
pub async fn get_example() -> impl Responder {
	let resource = ExampleResource { id: 1, name: "Example".to_string() };
	if let Err(validation_errors) = resource.validate() {
		return HttpResponse::BadRequest().json(
			super::models::ValidationErrorResponse {
				code: "validation.error".to_string(),
				message: "Validation failed".to_string(),
				description: format!("Validation error: {validation_errors}"),
				errors: std::collections::HashMap::new(),
			}
		);
	}
	HttpResponse::Ok().json(resource)
}
// Handler module for API endpoints.
// Move or implement your endpoint handlers here.

