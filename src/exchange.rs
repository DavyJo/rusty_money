use crate::currency::FormattableCurrency;
use crate::{Money, MoneyError};
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Stores `ExchangeRate`s for easier access.
#[derive(Debug, Default)]
pub struct Exchange<T: FormattableCurrency> {
    map: HashMap<String, ExchangeRate<T>>,
}

impl<T: FormattableCurrency> Exchange<T> {
    pub fn new() -> Exchange<T> {
        Exchange {
            map: HashMap::new(),
        }
    }

    /// Update an ExchangeRate or add it if does not exist.
    pub fn set_rate(&mut self, rate: &ExchangeRate<T>) {
        let key = Exchange::generate_key(&rate.from, &rate.to);
        self.map.insert(key, *rate);
    }

    /// Return the ExchangeRate given the currency pair.
    pub fn get_rate(&self, from: &T, to: &T) -> Option<ExchangeRate<T>> {
        let key = Exchange::generate_key(from, to);
        self.map.get(&key).copied()
    }

    fn generate_key(from: &T, to: &T) -> String {
        from.to_string() + "-" + &to.to_string()
    }
}

/// Stores rates of conversion between two currencies.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ExchangeRate<T: FormattableCurrency> {
    pub from: T,
    pub to: T,
    rate: Decimal,
}

impl<T: FormattableCurrency> ExchangeRate<T> {
    pub fn new(from: T, to: T, rate: Decimal) -> Result<ExchangeRate<T>, MoneyError> {
        if from == to {
            return Err(MoneyError::InvalidCurrency);
        }
        Ok(ExchangeRate { from, to, rate })
    }

    /// Converts a Money from one Currency to another using the exchange rate.
    pub fn convert(&self, amount: &Money<T>) -> Result<Money<T>, MoneyError> {
        if amount.currency() != self.from {
            return Err(MoneyError::InvalidCurrency);
        }
        let converted_amount = amount.amount() * self.rate;
        Ok(Money::from_decimal(converted_amount, self.to))
    }
}
