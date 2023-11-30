use base_project::configuration::get_configuration;
use base_project::startup::run;
use base_project::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("base_project".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Unable to read configuration file");
    let db_connection_pool =
        PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    let port_picked = listener.local_addr().unwrap().port();
    dbg!(format!("Listen on port {port_picked}"));

    run(listener, db_connection_pool)?.await
}
