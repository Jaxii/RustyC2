use actix_web::{middleware, web, HttpServer};
use tera::Tera;

mod routes;
mod settings;

use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: settings::FrontEndSettings = settings::FrontEndSettings::new();
}

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
    .bind(format!("{}:{}", CONFIG.bind_host, CONFIG.bind_port))
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