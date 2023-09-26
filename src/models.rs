//! Module for dealing with query validations 

use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum ValidationError {
    EMPTY_FIELD,
    INVALID_CHARSET(String),
    INVALID_CVV(String),
    INVALID_SPIRY_DATE(String),
    INVALID_PAN,
    INVALID_BITCHECK
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EMPTY_FIELD => write!(f, "This field cannot be empty"),
            ValidationError::INVALID_CHARSET(context) => write!(f, "Invalid Character found. {context}"),
            ValidationError::INVALID_CVV(context) => write!(f, "invalid lengt of CVV: {context}"),
            ValidationError::INVALID_SPIRY_DATE(content) => write!(f, "Expiration date must be later than today. {content}"),
            ValidationError::INVALID_PAN => write!(f, "Credit card numbers are 16 to 19 characters long"),
            ValidationError::INVALID_BITCHECK => write!(f, "Invalid credit card number"),

        }
    }
}

impl std::error::Error for ValidationError {}


/// This describes form fields that are to be validated
#[derive(Debug, Serialize, Deserialize)]
pub struct CCFields {
    name : String,
    ccn : String,
    expiry_date : String,
    cvv : String, 
}

impl CCFields {

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

        let today = Utc::now();

        let this_year = today.year();
        let this_month = today.month();

        let mut processed_date = self.expiry_date.split_ascii_whitespace();

        if let Some(given_year_str) = processed_date.next() {

            if let Some(given_month_str) = processed_date.next() {

                // Deal with leading zeores
                let given_month_str = given_month_str.trim_matches('0');

                let given_year = given_year_str .parse::<u32>();
                let given_month = given_month_str.parse::<u32>();

                return match (given_year.clone(), given_month.clone()) {
                    (Ok(year), Ok(month)) => {
                        if (this_year as u32) < year || (this_year as u32 == year && this_month < month)  {
                            None
                        }
                        else {
                            Some(ValidationError::INVALID_SPIRY_DATE("Date format is: %YYYY %mm".to_string()))
                        }
                    }
                    _ =>  {
                        Some(ValidationError::INVALID_SPIRY_DATE("Date format is: %YYYY %mm".to_string()))
                    }
                }
            }
        }

        // If given date is impropperly formatted, act accordingly
        return Some(ValidationError::INVALID_SPIRY_DATE("Date format is: %YYYY %mm".to_string()))
    }

    /// Validates CVV
    fn is_valid_cvv(self : &Self) -> Option<ValidationError> {
        // Validate charset
        if ! (self.cvv.chars().all(|sym| sym.is_ascii() && sym.is_numeric())) {
            Some(ValidationError::INVALID_CHARSET("Only numbers".to_string()))
        }
        // Validate correct CVV
        else if self.ccn.starts_with("34") || self.ccn.starts_with("37") {
            if self.cvv.len() != 4 {
                Some(ValidationError::INVALID_CVV("American Express cards have 4 digit CVVs".to_string())) 
            }
            else {
                None
            }
        }
        else {
            if self.cvv.len() != 3 {
                Some(ValidationError::INVALID_CVV("Regular credit cards have 3 digit CVVs".to_string())) 
            }
            else {
                None
            }
        }
    }

    /// Validates PAN
    fn is_valid_PAN(self : & Self) -> Option<ValidationError> {

        // Validate charset
        if ! (self.ccn.chars().all(|sym| sym.is_ascii() && sym.is_numeric())) {
            return Some(ValidationError::INVALID_CHARSET("Only numbers".to_string()))
        }
        
        let size= self.ccn.chars().count();
        if size >= 16 && size < 20 {
            None
        }
        else {
            Some(ValidationError::INVALID_PAN)
        }
    }

    /// Uses Lun algorithm
    fn bitcheck(self : &Self) -> Option<ValidationError> { 
        // Check charset
        if ! self.ccn.chars().all(|sym| sym.is_ascii() && sym.is_numeric()) {
            return Some( ValidationError::INVALID_BITCHECK)
        }
        
        // Run Lun's algorithm
        let mut payload = self.ccn.clone();

        let payload_after_sum = payload.chars()
            .rev()
            .map(|sym| sym as usize)
            .enumerate()
            .fold(0, |acc, (idx, val)| {
                if idx % 2 == 1 {
                    let exp = 2 * val;
                    let to_digit = if exp > 0 && exp % 9 == 0 { 9 } else { exp % 9 };
                    acc + to_digit 
                }
                else {
                    val
                }
            });


        if payload_after_sum  % 10 != 0 {
            Some( ValidationError::INVALID_BITCHECK)
        }
        else {
            None
        }
    }

    
}

/// Intermediate structure for checks over all form values
#[derive(Debug)]
pub struct AfterValidate {
    pub spiry_date : Option<ValidationError> ,
    pub cvv : Option<ValidationError>,
    pub pan : Option<ValidationError>, 
    pub bitcheck : Option<ValidationError>
}

impl AfterValidate {
    /// Checks all validations passed
    pub fn all_ok(self : &Self) -> bool {
        self.spiry_date.is_none() && self.cvv.is_none() && self.pan.is_none() && self.bitcheck.is_none()
    }

    /// Facility to turn Instances into strings
    pub fn err_to_string(inp : Option<ValidationError>) -> String {
        match inp {
            Some(err) => err.to_string(),
            None => "".to_string()
        }
    }
}

impl From<CCFields> for AfterValidate {
    fn from(value: CCFields) -> Self {
        value.run_validations()
    }
}