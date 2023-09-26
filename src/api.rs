//! API endpoints go here

use actix_web::{get, HttpResponse, Responder, web::Query, http::header::ContentType};
use askama::Template;

use crate::models::{CCFields, AfterValidate};

#[derive(Template, Default)]
#[template(path = "index.html")]
pub struct Index {
    spiry_date : String,
    cvv : String ,
    pan : String, 
    bitcheck : String, 
}

#[derive(Template, Default)]
#[template(path = "success.html")]
pub struct Success {}

/// Function holding behaviour for the single endpoint.
/// Perform validations and produce an HTML response accordingly
#[get("/")]
pub async fn unique(query : Option<Query<CCFields>>) -> impl Responder {
    
    log::info!("{:#?}", query);
    
    // Accordig to received parameters, produce an adecuate template
    let result = match query { 

        // Validation happen here
        Some(actual_query) => {
            log::info!("Validating query: {:#?}", actual_query);

            let cc_fields = actual_query.0;

            let validations = AfterValidate::from(cc_fields);

            if validations.all_ok() {
                // When all validations pass, render success temlate
                Success{}.render().expect("Problem with template")
            }
            else { 
                // Otherwise update Index state
                Index::from(validations).render().expect("Problem with template")
            }

        }
        // Direct Rendering
        None => {
            log::info!("Showing default view");
            Index{..Default::default() }.render().expect("invalid template")
        }
    };

    return HttpResponse::Ok()
            .insert_header(ContentType(mime::TEXT_HTML))
            .body(result)

}

impl From<AfterValidate> for Index {
    fn from(value: AfterValidate) -> Self {

        Index { 
            cvv: AfterValidate::err_to_string(value.cvv),
            spiry_date: AfterValidate::err_to_string(value.spiry_date),
            pan: AfterValidate::err_to_string(value.pan), 
            bitcheck: AfterValidate::err_to_string(value.bitcheck) 
        } 
    }
}