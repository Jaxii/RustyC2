use actix_web::{web, App, HttpResponse, HttpServer};
use lazy_static::lazy_static;

mod settings;

lazy_static!
{
    static ref CONFIG: settings::Settings =
        settings::Settings::new().unwrap();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    // Create HTTP listener for new agents
    let test_listener = HttpServer::new(|| App::new().service(web::resource("/").to(|| HttpResponse::Ok())));

    // Start HTTP listener
    let listener_addr = String::from(&format!(
        "{}:{}",
        CONFIG.listener.address, CONFIG.listener.port
    ));
    test_listener.bind(listener_addr)?
        .run()
        .await
}
