use actix_web::{get, web, HttpResponse, Responder};
use crate::models::{SingleResource, Meta};
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ExampleResource {
    pub id: String,
    pub name: String,
    pub other: String,
    pub some_object: Option<SomeObject>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SomeObject {
    pub some: String,
}

#[utoipa::path(
    get,
    path = "/example",
    responses(
        (status = 200, description = "Success", body = SingleResource<ExampleResource>),
        (status = 4XX, description = "Client error", body = ErrorResponse),
        (status = 5XX, description = "Server error", body = ErrorResponse),
        (status = 422, description = "Validation error", body = ValidationErrorResponse)
    )
)]
#[get("/example")]
pub async fn get_example() -> impl Responder {
    let resource = ExampleResource {
        id: "1".to_string(),
        name: "Example".to_string(),
        other: "Other".to_string(),
        some_object: Some(SomeObject { some: "value".to_string() }),
    };
    let meta = Meta { extra: Default::default() };
    let response = SingleResource { data: resource, meta };
    HttpResponse::Ok().json(response)
}
