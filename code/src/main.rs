use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    // Start listener
    HttpServer::new(
        || App::new()
            .service(web::resource("/").to(|| HttpResponse::Ok())))
        .bind("127.0.0.1:59090")?
        .run()
        .await
}
