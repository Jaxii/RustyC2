use actix_web::get;
use actix_web::HttpResponse;

#[get("/listeners")]
pub async fn get_listeners() -> HttpResponse {
    HttpResponse::Ok().await.unwrap()
}

#[get("/implants")]
pub async fn get_implants() -> HttpResponse {
    HttpResponse::Ok().await.unwrap()
}

#[get("/tasks")]
pub async fn get_tasks() -> HttpResponse {
    HttpResponse::Ok().await.unwrap()
}
