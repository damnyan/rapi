use actix_web::{get, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ExampleResource {
	pub id: i32,
	pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SomeObject {
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
	HttpResponse::Ok().json(ExampleResource { id: 1, name: "Example".to_string() })
}
// Handler module for API endpoints.
// Move or implement your endpoint handlers here.

