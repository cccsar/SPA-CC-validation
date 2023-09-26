//! API model for Json requests

use chrono::prelude::{DateTime, Utc, FixedOffset, NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum ValidationError {
    INVALID_CVV,
    INVALID_SPIRY_DATE,
    INVALID_PAN,
    INVALID_BITCHECK
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::INVALID_CVV => write!(f, "CVV must be 3 or 4 digits long"),
            ValidationError::INVALID_SPIRY_DATE => write!(f, "Expiration date must be later than today"),
            ValidationError::INVALID_PAN => write!(f, "Credit card numbers are 16 to 19 characters long"),
            ValidationError::INVALID_BITCHECK => write!(f, "Invalid credit card number"),

        }
    }
}

impl std::error::Error for ValidationError {}


#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct CCFields {
    name : String,
    ccn : String,
    expiry_date : String, 
    cvv : String, 
}

impl CCFields {
    //! Validations for API responses

    pub fn run_validations(self : & Self) -> AfterValidate {
        AfterValidate { 
            spiry_date: self.is_valid_spiry_date(), 
            cvv: self.is_valid_cvv(), 
            pan: self.is_valid_PAN() ,
            bitcheck: self.bitcheck()
        }        
    }

    /// Validates expiration date
    fn is_valid_spiry_date(self: &Self) -> Option<ValidationError> { 
        // let given_date = NaiveDate::parse_from_str(date, "%Y %m %d").unwrap(); 
        todo!();
    }
    // TODO add propper error handling

    // Validates CVV
    fn is_valid_cvv(self : &Self) -> Option<ValidationError> {
        todo!();
    }
    // TODO finish validations
    // TODO add propper error handling

    // Validates PAN
    fn is_valid_PAN(self : & Self) -> Option<ValidationError> {
        let size= self.ccn.chars().count();

        
        if size >= 16 && size < 20 {
            None
        }
        else {
            Some(ValidationError::INVALID_PAN)
        }
    }
    // TODO add propper error handling

    // Uses Lun algorithm
    fn bitcheck(self : &Self) -> Option<ValidationError> { 
        todo!();
    }
    // TODO Implement Lun's algorithm 
    // TODO add propper error handling
}

#[derive(Debug)]
pub struct AfterValidate {
    spiry_date : Option<ValidationError> ,
    cvv : Option<ValidationError>,
    pan : Option<ValidationError>, 
    bitcheck : Option<ValidationError>
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_spiry_val() {
        todo!();
    }

    #[test]
    pub fn test_valid_cvv() {
        todo!();
    }
    #[test]
    pub fn test_valid_PAN() {
        todo!();
    }
}