use actix_web::{get, rt::System, web, HttpRequest, HttpResponse, Responder};
use awc::Client;
use lazy_static::lazy_static;
use serde_json;
use tera::{Context, Tera};

use crate::settings;

lazy_static! {
    static ref CONFIG: settings::FrontEndSettings = settings::FrontEndSettings::new();
}

#[get("/")]
pub async fn root() -> impl Responder {
    return HttpResponse::MovedPermanently()
        .append_header(("Location", "/dashboard"))
        .finish();
}

#[get("/dashboard")]
pub async fn dashboard(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "RustyC2 - Dashboard");

    let rendered = tera.render("dashboard.html", &data).unwrap();
    return HttpResponse::Ok().body(rendered);
}

#[get("/terminal/{id}")]
pub async fn terminal(tera: web::Data<Tera>, req: HttpRequest) -> impl Responder {
    match req.match_info().query("id").parse::<u16>() {
        Ok(listener_id) => {
            let mut data = Context::new();
            data.insert("title", "RustyC2 - Terminal");

            System::new().block_on(async {
                let client = Client::default();

                let res = client
                    .get(format!(
                        "http://{}:{}/listeners/{}",
                        CONFIG.api_server.host, CONFIG.api_server.port, listener_id
                    )) // <- Create request builder
                    .send() // <- Send http request
                    .await;

                match res {
                    Ok(mut http_response) => {
                        let _json_response = http_response.json::<serde_json::Value>();
                        let rendered = tera.render("dashboard.html", &data).unwrap();
                        return HttpResponse::Ok().body(rendered);
                    }
                    Err(_) => {
                        log::error!("Failed to get the specified listener");
                        return HttpResponse::NotFound().await.unwrap();
                    }
                }
            });

            let rendered = tera.render("dashboard.html", &data).unwrap();
            return HttpResponse::Ok().body(rendered);
        }
        Err(_) => {
            return HttpResponse::NotFound().await.unwrap();
        }
    }
}
