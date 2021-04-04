mod config;

use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = config::Config::from_env().expect("error loading .env");

    HttpServer::new(|| App::new().wrap(Logger::default()))
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;

    Ok(())
}
