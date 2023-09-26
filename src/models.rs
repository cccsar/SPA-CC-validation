//! API model for Json requests

use serde::{Serialize, Deserialize};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct CCFields {
    expiry_date : String, 
    cvv : String, 
    pan : String
}