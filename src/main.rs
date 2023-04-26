use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
