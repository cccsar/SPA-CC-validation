use actix_web::{ HttpServer, App, web};
use tokio;
use anyhow;

mod api;
mod models;
mod validations;


#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                    .service(api::unique)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
// TODO add API documentation

