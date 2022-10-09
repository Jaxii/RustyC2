use actix_web::{middleware, HttpServer};

#[actix_web::main]
pub async fn start_api_server() {
    match HttpServer::new(|| {
        actix_web::App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(super::routes::get_listeners)
            .service(super::routes::get_listener)
    })
    .bind("0.0.0.0:9090")
    .unwrap()
    .run()
    .await
    {
        Ok(_) => {
            println!("[+] API Server started successfully");
        }
        Err(_) => {
            println!("[!] Failed to start the API Server");
        }
    }
}
