use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn healt_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Notice the signature!
// We return 'Server'on the happy path and we dont use async
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(healt_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
