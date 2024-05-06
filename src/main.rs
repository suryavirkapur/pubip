use std::str::FromStr;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn getip(req: HttpRequest) -> impl Responder {
    let con_info = req.connection_info().clone();
    let ip = con_info.realip_remote_addr().unwrap();
    let formed_ip = String::from_str(ip).unwrap();
    HttpResponse::Ok().body(formed_ip)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(getip)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
