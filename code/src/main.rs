use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    // Create HTTP listener for new agents
    let test_listener = HttpServer::new(|| App::new().service(web::resource("/").to(|| HttpResponse::Ok())));

    // Start HTTP listener
    test_listener.bind("0.0.0.0:4444")?
        .run()
        .await
}
