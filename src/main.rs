mod handlers;
mod models;

use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health::health_check,
    ),
    components(
        schemas()
    ),
    tags(
        (name = "health", description = "Health check endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    println!("Swagger UI available at http://localhost:8080/swagger-ui/");

    HttpServer::new(|| {
        App::new()
            .service(handlers::health::health_check)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
