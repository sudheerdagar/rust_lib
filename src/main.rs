use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};

enum Services {
    AWS(String),
    GCP(String),
    Azure(String),
}

struct ServiceProvider {
    service: Option<Services>,
}

impl ServiceProvider {
    fn set_service(&mut self, service: Services) {
        self.service = Some(service);
    }
}

async fn set_service_provider(
    service: web::Json<Services>,
    data: web::Data<Arc<Mutex<ServiceProvider>>>,
) -> impl Responder {
    let mut service_provider = data.lock().unwrap();
    service_provider.set_service(service.into_inner());
    "Service provider set".to_string()
}

async fn test(
    data: web::Data<Arc<Mutex<ServiceProvider>>>,
) -> impl Responder {
    let service_provider = data.lock().unwrap();
    if let Some(ref service) = service_provider.service {
        HttpResponse::Ok().body(format!("{:?}", service))
    } else {
        HttpResponse::NotFound().body("Service provider not set")
    }
}

#[get("/hi")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("Working fine !")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let service_provider = Arc::new(Mutex::new(ServiceProvider { service: None }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service_provider.clone()))
            .route("/set-service", web::post().to(set_service_provider))
            .service(web::resource("/test").route(web::get().to(test)))
            .service(hi)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
