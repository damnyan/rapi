
use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod models;
pub mod api;
pub mod errors;

#[derive(OpenApi)]
#[openapi(
    paths(api::get_example),
    components(schemas(
        models::ErrorResponse,
        models::ValidationErrorResponse,
        models::PaginatedList<api::ExampleResource>,
        models::SingleResource<api::ExampleResource>,
        models::PaginationMeta,
        models::Meta,
        api::ExampleResource,
        api::SomeObject
    )),
    tags()
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::get_example)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
