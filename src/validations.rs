//! Validations for API responses

use chrono::prelude::{DateTime, Utc, FixedOffset, NaiveDate};

/// Validates
pub fn is_valid_spiry_date(date : &str) -> bool { 
    // let given_date = NaiveDate::parse_from_str(date, "%Y %m %d").unwrap(); 
    todo!();
}
// TODO add propper error handling

pub fn is_valid_cvv(ccn : &str, cvv: &str) -> bool {
    todo!();
}
// TODO finish validations
// TODO add propper error handling

pub fn is_valid_PAN(pan : &str) -> bool {
    let size= pan.chars().count();

    return size >= 16 && size < 20;
}
// TODO add propper error handling

pub fn bitcheck(ccn : &str) -> bool { 
    todo!();
}
// TODO Implement Lun's algorithm 
// TODO add propper error handling

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