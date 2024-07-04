use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use serde::Deserialize;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// 通过声明这一个struct，就可以实现自动的数据匹配和报错
// 原理是FormRequest trait
// 书3.7.3.1
#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
