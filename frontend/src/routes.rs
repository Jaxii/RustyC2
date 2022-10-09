use actix_web::{get, web, HttpResponse, Responder};
use lazy_static::lazy_static;
use tera::{Context, Tera};

use crate::settings;

lazy_static! {
    static ref CONFIG: settings::FrontEndSettings = settings::FrontEndSettings::new();
}

#[get("/dashboard")]
pub async fn dashboard(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "RustyC2 - Dashboard");

    let rendered = tera.render("dashboard.html", &data).unwrap();
    return HttpResponse::Ok().body(rendered);
}
