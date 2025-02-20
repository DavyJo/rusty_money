use crate::{Money, Round};
use std::cmp::Ordering;

/// Converts Money objects into human readable strings.
pub struct Formatter;

impl Formatter {
    /// Returns a formatted Money String given parameters and a Money object.
    pub fn money(money: &Money, params: Params) -> String {
        let mut decimal = *money.amount();

        // Round the decimal
        if let Some(x) = params.rounding {
            decimal = *money.round(x, Round::HalfEven).amount();
        }

        // Format the Amount String
        let amount = Formatter::amount(&format!("{}", decimal), &params);

        // Position values in the Output String
        let mut result = String::new();
        for position in params.positions.iter() {
            match position {
                Position::Space => result.push(' '),
                Position::Amount => result.push_str(&amount),
                Position::Code => result.push_str(params.code.unwrap_or("")),
                Position::Symbol => result.push_str(params.symbol.unwrap_or("")),
                Position::Sign => result.push_str(if money.is_negative() { "-" } else { "" }),
            }
        }
        result
    }

    /// Returns a formatted amount String, given the raw amount and formatting parameters.
    fn amount(raw_amount: &str, params: &Params) -> String {
        // Split amount into digits and exponent.
        let amount_split: Vec<&str> = raw_amount.split('.').collect();
        let mut amount_digits = amount_split[0].to_string();

        // Format the digits
        amount_digits.retain(|c| c != '-');
        amount_digits = Formatter::digits(
            &amount_digits,
            params.digit_separator,
            &params.separator_pattern,
        );
        let mut result = amount_digits;

        // Format the exponent, and add to digits
        match amount_split.len().cmp(&2) {
            Ordering::Equal => {
                // Exponent found, concatenate to digits.
                result.push(params.exponent_separator);
                result += amount_split[1];
            }
            Ordering::Less => {
                // No exponent, do nothing.
            }
            Ordering::Greater => panic!("More than 1 exponent separators when parsing Decimal"),
        }

        result
    }

    /// Returns a formatted digit component, given the digit string, separator and pattern of separation.
    fn digits(raw_digits: &str, separator: char, pattern: &[usize]) -> String {
        let mut digits = raw_digits.to_string();

        let mut current_position: usize = 0;
        for position in pattern.iter() {
            current_position += position;
            if digits.len() > current_position {
                digits.insert(digits.len() - current_position, separator);
                current_position += 1;
            }
        }
        digits
    }
}

/// Items which must be positioned in a Money string.
#[derive(Debug, Clone)]
pub enum Position {
    Space,
    Amount,
    Code,
    Symbol,
    Sign,
}

/// Group of formatting parameters consumed by `Formatter`.
#[derive(Debug, Clone)]
pub struct Params {
    /// The character that separates grouped digits (e.g. 1,000,000)
    pub digit_separator: char,
    /// The character that separates minor units from major units (e.g. 1,000.00)
    pub exponent_separator: char,
    /// The grouping pattern that is applied to digits / major units (e.g. 1,000,000 vs 1,00,000)
    pub separator_pattern: Vec<usize>,
    /// The relative positions of the elements in a currencies string (e.g. -$1,000 vs $ -1,000)
    pub positions: Vec<Position>,
    /// The number of minor unit digits should remain after Round::HalfEven is applied.
    pub rounding: Option<u32>,
    /// The symbol of the currencies (e.g. $)
    pub symbol: Option<&'static str>,
    /// The currencies's ISO code (e.g. USD)
    pub code: Option<&'static str>,
}

impl Default for Params {
    /// Defines the default parameters to format a Money string.
    fn default() -> Params {
        Params {
            digit_separator: ',',
            exponent_separator: '.',
            separator_pattern: vec![3, 3, 3],
            positions: vec![Position::Sign, Position::Symbol, Position::Amount],
            rounding: None,
            symbol: None,
            code: None,
        }
    }
}
