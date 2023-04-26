use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::{healt_check, subscribe};
// Notice the signature!
// We return 'Server'on the happy path and we dont use async
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(healt_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
