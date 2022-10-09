use actix_web::HttpResponse;
use actix_web::{get, HttpRequest};

use crate::database;
use crate::models::HTTPListener;

static APPLICATION_JSON: &str = "application/json";

#[get("/listeners")]
pub async fn get_listeners() -> HttpResponse {
    // TODO: join vector with other listener vectors, based on their protocols
    // TODO: sort based on their ID
    let http_listeners: Vec<HTTPListener> = database::get_http_listeners();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(http_listeners)
}

#[get("/listeners/{id}")]
pub async fn get_listener(req: HttpRequest) -> HttpResponse {
    match req.match_info().query("id").parse::<u16>() {
        Ok(listener_id) => match database::get_http_listener(listener_id) {
            Some(http_listener) => {
                return HttpResponse::Ok()
                    .content_type(APPLICATION_JSON)
                    .json(http_listener);
            }
            None => return HttpResponse::NotFound().await.unwrap(),
        },
        Err(_) => return HttpResponse::NotFound().await.unwrap(),
    };
}

#[get("/implants")]
pub async fn get_implants() -> HttpResponse {
    HttpResponse::Ok().await.unwrap()
}

#[get("/tasks")]
pub async fn get_tasks() -> HttpResponse {
    HttpResponse::Ok().await.unwrap()
}
