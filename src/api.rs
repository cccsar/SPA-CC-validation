//! API endpoints go here

use actix_web::{get, HttpResponse, Responder, web::Query, http::header::ContentType};
use askama::Template;

use crate::models::CCFields;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

/// Function holding behaviour for the single endpoint.
/// Perform validations and produce an HTML response accordingly
#[get("/")]
pub async fn unique(query : Option<Query<CCFields>>) -> impl Responder {
    
    // Accordig to received parameters, produce an adecuate template
    let result = match query { 
        Some(actual_query) => {
            // Validation happen here
            let _validations = actual_query.0.run_validations();

            "Implementing validations".to_string()
        }
        None => {
            // Direct Rendering
            Index{}.render().expect("invalid template")
        }
    };

    return HttpResponse::Ok()
            .insert_header(ContentType(mime::TEXT_HTML))
            .body(result)

}
// TODO Expiry date val
// TODO CVV val
// TODO Pan val
// TODO Lun's algorithm bit check
