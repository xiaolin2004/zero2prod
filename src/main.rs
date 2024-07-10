use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::{Connection, PgPool};

use zero2prod::configuration::get_configuration;
use zero2prod::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Fail to read configurations");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string.expose_secret())
        .await
        .expect("Failed to connect Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

