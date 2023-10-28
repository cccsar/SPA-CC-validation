// External
use actix_web::{ HttpServer, App, middleware::Logger};
use actix_files as fs;
use env_logger::Env;
use tokio;
use anyhow;

// Local
mod api;
mod models;

/// This ideally would be environment
const SERVER_ADDR : &str = "127.0.0.1";
const SERVER_PORT : u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()>  {

    // Initialize logger
    env_logger::init_from_env( Env::default().default_filter_or("info") );

    log::info!("👾 Starting server at http://{SERVER_ADDR}:{SERVER_PORT}");

    // Initialize server
    HttpServer::new(|| {
        App::new()

            // Serving static files
            .service(fs::Files::new("/static", "./static"))    

            // Add logger
            .wrap(Logger::new("%a %{User-Agent}i"))

            // Add route
            .service(api::unique)

    })
    .bind((SERVER_ADDR, SERVER_PORT))?  
    .run()
    .await?;

    Ok(())
}