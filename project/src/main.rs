use actix_web::{web, App, HttpServer};
use tracing_subscriber::fmt;
mod db;
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
    // Initialize structured logging with tracing
    fmt().init();

    let pool = db::create_pool();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::get_example)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/openapi.json", ApiDoc::openapi()))
    })
    .bind(("0.0.0.0", 8080))
    .map_err(|e| {
        tracing::error!(error = %e, "Failed to bind server");
        e
    })?
    .run()
    .await
}
