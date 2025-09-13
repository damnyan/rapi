use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod errors;
pub mod handlers;
pub mod models;

#[derive(OpenApi)]
#[openapi(
    paths(handlers::get_example),
    components(schemas(
        models::ErrorResponse,
        models::ValidationErrorResponse,
    models::PaginatedList<handlers::ExampleResource>,
    models::SingleResource<handlers::ExampleResource>,
        models::PaginationMeta,
        models::Meta,
    handlers::ExampleResource,
    handlers::SomeObject
    )),
    tags()
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::get_example)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/openapi.json", ApiDoc::openapi()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
