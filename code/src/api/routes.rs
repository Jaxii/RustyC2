use actix_web::HttpResponse;
use actix_web::{get, HttpRequest};

use crate::database;
use crate::models::GenericListener;

#[get("/listeners")]
pub async fn get_listeners() -> HttpResponse {
    let listeners: Vec<GenericListener> = crate::database::get_listeners();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(listeners)
}

#[get("/listeners/{id}")]
pub async fn get_listener(req: HttpRequest) -> HttpResponse {
    match req.match_info().query("id").parse::<u16>() {
        Ok(listener_id) => match database::get_listener(listener_id) {
            Some(generic_listener) => {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .json(generic_listener)
            }
            None => return HttpResponse::NotFound().await.unwrap(),
        },
        Err(_error) => return HttpResponse::NotFound().await.unwrap(),
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
