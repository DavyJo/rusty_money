use crate::Money; // Removed Round from here
use std::cmp::Ordering;

/// Converts Money objects into human readable strings.
///
/// The `Formatter` provides a flexible and configurable way to format Money objects
/// for display purposes, handling various locale-specific formatting requirements
/// such as digit grouping, symbol placement, and rounding.
///
/// # Rounding Behavior
///
/// When rounding is specified in the `Params`, the `Formatter` uses the
/// `MidpointNearestEven` rounding strategy (also known as "banker's rounding").
/// This strategy rounds to the nearest value and to the even digit if equidistant.
///
/// Examples of `MidpointNearestEven` rounding to 2 decimal places:
/// - 1.225 → 1.22 (rounds down because 2 is even)
/// - 1.235 → 1.24 (rounds up because 4 is even)
/// - 1.245 → 1.24 (rounds down because 4 is even)
/// - 1.255 → 1.26 (rounds up because 6 is even)
///
/// Note that this differs from the rounding used in `Money::fmt` (Display trait),
/// which uses `MidpointAwayFromZero` (commercial rounding).
pub struct Formatter;

impl Formatter {
    /// Returns a formatted Money String given parameters and a Money object.
    ///
    /// This method applies the specified formatting parameters to the Money object,
    /// including digit grouping, symbol placement, and optional rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_money::{Money, Formatter, Params, Position};
    /// use rusty_money::currencies::iso::USD;
    /// use rust_decimal_macros::dec;
    ///
    /// let money = Money::from_decimal(dec!(1234.56), USD);
    ///
    /// // Custom formatting with specific parameters
    /// let params = Params {
    ///     symbol: Some("$"),
    ///     positions: vec![Position::Symbol, Position::Amount],
    ///     rounding: Some(1), // Round to 1 decimal place
    ///     ..Default::default()
    /// };
    ///
    /// assert_eq!("$1,234.6", Formatter::money(&money, params));
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the input decimal representation contains more
    /// than one decimal point (exponent separator), which should never occur with
    /// a valid `Decimal` value from a `Money` object.
    pub fn money(money: &Money, params: Params) -> String {
        let mut decimal = *money.amount();

        // Round the decimal using MidpointNearestEven (equivalent to the old HalfEven)
        if let Some(x) = params.rounding {
             // Use the imported RoundingStrategy enum
            decimal = *money.round(x, rust_decimal::RoundingStrategy::MidpointNearestEven).amount();
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
///
/// The `Params` struct allows fine-grained control over how Money values
/// are formatted, including digit grouping, decimal representation,
/// symbol placement, and rounding behavior.
#[derive(Debug, Clone)]
pub struct Params {
    /// The character that separates grouped digits (e.g., 1,000,000 uses ',' as the separator)
    pub digit_separator: char,
    
    /// The character that separates major units from minor units (e.g., 1,000.00 uses '.' as the separator)
    pub exponent_separator: char,
    
    /// The grouping pattern applied to digits/major units
    ///
    /// For standard grouping like 1,000,000, use [3, 3, 3]
    /// For Indian grouping like 1,00,00,000, use [3, 2, 2]
    pub separator_pattern: Vec<usize>,
    
    /// The relative positions of the elements in a currency string
    ///
    /// This determines the order of elements like:
    /// - "-$1,000" (Sign, Symbol, Amount)
    /// - "$ -1,000" (Symbol, Space, Sign, Amount)
    /// - "1,000$" (Amount, Symbol)
    pub positions: Vec<Position>,
    
    /// The number of decimal places to round to using `MidpointNearestEven` rounding
    ///
    /// When `Some(n)`, the amount will be rounded to `n` decimal places using banker's rounding.
    /// When `None`, no rounding is performed (all decimal places are preserved).
    ///
    /// Examples with `rounding: Some(2)`:
    /// - 10.005 → 10.00
    /// - 10.015 → 10.02
    /// - 10.025 → 10.02
    /// - 10.035 → 10.04
    pub rounding: Option<u32>,
    
    /// The symbol of the currency (e.g., "$", "€", "£")
    pub symbol: Option<&'static str>,
    
    /// The currency's ISO code (e.g., "USD", "EUR", "GBP")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currencies::iso::USD;
    use crate::find_currency;

    #[test]
    fn format_position() {
        let _usd = find_currency("USD"); // Prevents unused code warnings from the defined module.

        let money = Money::from_major(-1000, USD);

        // Test that you can position eSpace, Amount, Code, Symbol and Sign in different places
        let params = Params {
            symbol: Some("$"),
            code: Some("USD"),
            positions: vec![
                Position::Sign,
                Position::Space,
                Position::Symbol,
                Position::Amount,
                Position::Space,
                Position::Code,
            ],
            ..Default::default()
        };
        assert_eq!("- $1,000 USD", Formatter::money(&money, params));

        let params = Params {
            symbol: Some("$"),
            code: Some("USD"),
            positions: vec![
                Position::Code,
                Position::Space,
                Position::Amount,
                Position::Symbol,
                Position::Space,
                Position::Sign,
            ],
            ..Default::default()
        };
        assert_eq!("USD 1,000$ -", Formatter::money(&money, params));

        // Test that you can omit some, and it works fine.
        let params = Params {
            positions: vec![Position::Amount],
            ..Default::default()
        };
        assert_eq!("1,000", Formatter::money(&money, params));

        let params = Params {
            symbol: Some("$"),
            positions: vec![Position::Symbol],
            ..Default::default()
        };
        assert_eq!("$", Formatter::money(&money, params));

        // Missing Optionals Insert Nothing
        let params = Params {
            positions: vec![Position::Amount, Position::Symbol],
            ..Default::default()
        };
        assert_eq!("1,000", Formatter::money(&money, params));
    }

    #[test]
    fn format_digit_separators_with_custom_separators() {
        let params = Params {
            digit_separator: '/',
            ..Default::default()
        };

        // For 1_000_000
        let money = Money::from_major(1_000_000, USD);
        assert_eq!("1/000/000", Formatter::money(&money, params.clone()));

        // For 1_000
        let money = Money::from_major(1_000, USD);
        assert_eq!("1/000", Formatter::money(&money, params.clone()));

        // For 0 Chars
        let money = Money::from_major(0, USD);
        assert_eq!("0", Formatter::money(&money, params));
    }

    #[test]
    fn format_digit_separators_with_custom_sequences() {
        let params = Params {
            separator_pattern: vec![3, 2, 2],
            ..Default::default()
        };

        let money = Money::from_major(10_000_000, USD);
        assert_eq!("1,00,00,000", Formatter::money(&money, params.clone()));

        let money = Money::from_major(100_000, USD);
        assert_eq!("1,00,000", Formatter::money(&money, params.clone()));

        let money = Money::from_major(1_000, USD);
        assert_eq!("1,000", Formatter::money(&money, params));

        // With a zero sequence
        let params = Params {
            separator_pattern: vec![0, 2],
            ..Default::default()
        };

        let money = Money::from_major(100, USD);
        assert_eq!("1,00,", Formatter::money(&money, params.clone()));

        let money = Money::from_major(0, USD);
        assert_eq!("0,", Formatter::money(&money, params));
    }

    // What if pattern includes a zero or negative number?

    #[test]
    fn format_rounding() {
        let money = Money::from_minor(1000, USD) / 3;

        // Rounding = Some (0)
        let params = Params {
            rounding: Some(0),
            ..Default::default()
        };
        assert_eq!("3", Formatter::money(&money, params));

        // Rounding = Some(2)
        let params = Params {
            rounding: Some(2),
            ..Default::default()
        };
        assert_eq!("3.33", Formatter::money(&money, params));

        // Rounding = None
        let params = Params {
            ..Default::default()
        };
        assert_eq!(
            "3.3333333333333333333333333333",
            Formatter::money(&money, params)
        );
    }
}
