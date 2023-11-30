use base_project::configuration::get_configuration;
use base_project::startup::run;
use base_project::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("base_project".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Unable to read configuration file");
    let db_connection_pool =
        PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to create Postgres connection pool.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let port_picked = listener.local_addr().unwrap().port();
    dbg!(format!("Listen on port {port_picked}"));

    run(listener, db_connection_pool)?.await
}
