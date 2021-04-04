mod config;
mod handlers;

use crate::handlers::app_config;
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use tracing::info;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = config::Config::from_env().expect("error loading .env");

    info!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(app_config))
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;

    Ok(())
}
