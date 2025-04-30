use std::{error, fmt, num::ParseIntError}; // Add ParseIntError import

use rust_decimal::Error as DecimalError;

/// Standard Error type for this crate.
#[derive(Debug)] // Remove PartialEq, as ParseIntError doesn't impl it
pub enum MoneyError {
    /// Operation attempted on Money with different currencies.
    CurrencyMismatch,
    /// Provided currency code was not valid or recognized.
    InvalidCurrency,
    /// Could not parse the decimal part of the provided amount string. Wraps `rust_decimal::Error`.
    InvalidAmountDecimal(DecimalError),
    /// Could not parse an integer part (e.g., exponent) of the provided amount string. Wraps `std::num::ParseIntError`.
    InvalidAmountInteger(ParseIntError),
    /// Amount string format was invalid (e.g., wrong separators, multiple exponents).
    InvalidAmountFormat,
    /// Provided allocation ratios were invalid (e.g., empty, zero, negative).
    InvalidRatio,
    /// Attempted to sum an empty iterator of Money.
    EmptySum,
    /// Locale formatting string was invalid (indicates internal library error).
    InvalidLocaleFormat,
}

// Manual PartialEq implementation because ParseIntError doesn't derive it
impl PartialEq for MoneyError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::CurrencyMismatch, Self::CurrencyMismatch) => true,
            (Self::InvalidCurrency, Self::InvalidCurrency) => true,
            (Self::InvalidAmountDecimal(l), Self::InvalidAmountDecimal(r)) => l.to_string() == r.to_string(), // Compare DecimalError string representation
            (Self::InvalidAmountInteger(l), Self::InvalidAmountInteger(r)) => l == r,
            (Self::InvalidAmountFormat, Self::InvalidAmountFormat) => true,
            (Self::InvalidRatio, Self::InvalidRatio) => true,
            (Self::EmptySum, Self::EmptySum) => true,
            (Self::InvalidLocaleFormat, Self::InvalidLocaleFormat) => true,
            _ => false,
        }
    }
}


impl fmt::Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoneyError::CurrencyMismatch => write!(f, "Operation attempted on Money with different currencies"),
            MoneyError::InvalidCurrency => write!(f, "Provided currency code was not valid or recognized"),
            MoneyError::InvalidAmountDecimal(e) => write!(f, "Amount decimal part not parsable: {}", e),
            MoneyError::InvalidAmountInteger(e) => write!(f, "Amount integer part not parsable: {}", e),
            MoneyError::InvalidAmountFormat => write!(f, "Amount string format is invalid"),
            MoneyError::InvalidRatio => write!(f, "Provided allocation ratios were invalid"),
            MoneyError::EmptySum => write!(f, "Cannot sum an empty iterator of Money"),
            MoneyError::InvalidLocaleFormat => write!(f, "Locale formatting string was invalid"),
        }
    }
}

impl error::Error for MoneyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            MoneyError::InvalidAmountDecimal(e) => Some(e),
            MoneyError::InvalidAmountInteger(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseIntError> for MoneyError {
    fn from(err: ParseIntError) -> MoneyError {
        // Wrap the ParseIntError
        MoneyError::InvalidAmountInteger(err)
    }
}

impl From<DecimalError> for MoneyError {
    fn from(err: DecimalError) -> MoneyError {
         // Wrap the DecimalError
        MoneyError::InvalidAmountDecimal(err)
    }
}
