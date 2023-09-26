//! API endpoints go here

use actix_web::{get, HttpResponse, Responder, web::Json};

use crate::validations;

use crate::models::CCFields;

/// Function holding behaviour for the single endpoint.
/// Perform validations and act accordingly
#[get("/")]
pub async fn unique(input : Json<CCFields>) -> impl Responder {

    HttpResponse::Ok().body(format!("Hello from CC!. Fields :{:#?}", input))
    
}
// TODO Expiry date val
// TODO CVV val
// TODO Pan val
// TODO Lun's algorithm bit check
