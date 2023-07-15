use crate::{
    configuration::{DatabaseSettings, Settings},
    routes::{health_check, signup_view},
    telemetry::RouterExt,
};
use std::net::TcpListener;

use axum::{
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use secrecy::Secret;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub type AppServer = Server<AddrIncoming, IntoMakeService<Router>>;
pub struct Application {
    port: u16,
    server: AppServer,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            configuration.application.hmac_secret,
        )
        .await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), hyper::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: Secret<String>,
) -> Result<AppServer, anyhow::Error> {
    // Routes that need to not have a session applied
    let router_no_session = Router::new()
        .route("/health_check", get(health_check))
        .route("/signup", get(signup_view));

    // Create a router that will contain and match all routes for the application
    let app = Router::new()
        .merge(router_no_session)
        .with_state(db_pool)
        .with_state(base_url)
        .with_state(hmac_secret)
        .add_axum_tracing_layer();

    // Start the axum server and set up to use supplied listener
    Ok(axum::Server::from_tcp(listener)
        .expect("failed to create server from listener")
        .serve(app.into_make_service()))
}

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);
