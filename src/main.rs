use actix_web::{ HttpServer, App, web, middleware::Logger};
use env_logger::Env;
use tokio;
use anyhow;

mod api;
mod models;
mod validations;

/// This ideally would be environment
const SERVER_ADDR : &str = "127.0.0.1";
const SERVER_PORT : u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    // Initialize logger
    env_logger::init_from_env( Env::default().default_filter_or("info") );

    log::info!("👾 Starting server at {SERVER_ADDR} on port {SERVER_PORT}");

    HttpServer::new(|| {
        App::new()
            // Add route
            .service(
                web::scope("/api/v1")
                    .service(api::unique)
            )
            // Add logger
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind((SERVER_ADDR, SERVER_PORT))?  .run()
    .await?;

    Ok(())
}
// TODO add API documentation

