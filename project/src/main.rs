use actix_web::{middleware::Logger, web, App, HttpServer};
use tracing_subscriber::fmt::Subscriber as FmtSubscriber;
use tracing_subscriber::EnvFilter;
mod db;
mod middlewares;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod errors;
pub mod handlers;
pub mod models;

#[derive(OpenApi)]
#[openapi(
    paths(handlers::create_example),
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
    // Initialize structured logging with tracing, log level from RUST_LOG or default to info
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let pool = db::create_pool();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middlewares::TracingLogger)
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::create_example)
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
