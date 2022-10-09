use actix_web::{middleware, web, HttpServer};
use tera::Tera;
mod routes;

#[actix_web::main]
pub async fn main() {
    match HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();

        actix_web::App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(tera))
            // register HTTP requests handlers
            .service(routes::dashboard)
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
    {
        Ok(_) => {}
        Err(_) => {
            println!("[!] Failed to start the API Server");
        }
    }
}
