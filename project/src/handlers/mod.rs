use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use validator_derive::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug)]
pub struct ExampleResource {
    pub id: i32,
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug)]
pub struct SomeObject {
    #[validate(length(min = 1, message = "Value must not be empty"))]
    pub value: String,
}

#[utoipa::path(
	post,
	path = "/example",
	request_body = ExampleResource,
	responses(
		(status = 200, description = "Example response", body = ExampleResource),
		(status = 422, description = "Validation error", body = super::models::ValidationErrorResponse)
	)
)]
#[post("/example")]
pub async fn create_example(item: web::Json<ExampleResource>) -> impl Responder {
    let resource = item.into_inner();
    match resource.validate() {
        Ok(_) => HttpResponse::Ok().json(resource),
        Err(validation_errors) => {
            let mut errors = std::collections::HashMap::new();
            for (field, field_errors) in validation_errors.field_errors().iter() {
                let messages: Vec<String> = field_errors
                    .iter()
                    .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                    .collect();
                errors.insert(field.to_string(), messages);
            }
            HttpResponse::UnprocessableEntity().json(super::models::ValidationErrorResponse {
                code: "validation.error".to_string(),
                message: "Validation failed".to_string(),
                description: format!("Validation error: {validation_errors}"),
                errors,
            })
        }
    }
}

// Handler module for API endpoints.
// Move or implement your endpoint handlers here.
