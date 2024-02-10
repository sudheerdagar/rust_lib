use actix_web::{HttpServer, App};
mod api;
use crate::api::gcp::bigquery::bigquery_handler;
use crate::api::gcp::vertexai::vertexai_handler;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(bigquery_handler)
            .service(vertexai_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
