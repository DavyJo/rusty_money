use crate::{Currency, Money, MoneyError};
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Stores `ExchangeRate`s for easier access.
#[derive(Debug, Default)]
pub struct Exchange {
    map: HashMap<String, ExchangeRate>,
}

impl Exchange {
    pub fn new() -> Exchange {
        Exchange {
            map: HashMap::new(),
        }
    }

    /// Update an ExchangeRate or add it if does not exist.
    pub fn set_rate(&mut self, rate: &ExchangeRate) {
        let key = Exchange::generate_key(&rate.from, &rate.to);
        // Clone the rate since ExchangeRate is not Copy
        self.map.insert(key, rate.clone());
    }

    /// Return the ExchangeRate given the currency pair.
    pub fn get_rate(&self, from: &Currency, to: &Currency) -> Option<ExchangeRate> {
        let key = Exchange::generate_key(from, to);
        // Use cloned() instead of copied() as ExchangeRate is not Copy
        self.map.get(&key).cloned()
    }

    fn generate_key(from: &Currency, to: &Currency) -> String {
        from.to_string() + "-" + &to.to_string()
    }
}

/// Stores rates of conversion between two currencies.
#[derive(Debug, PartialEq, Clone)] // Removed Copy
pub struct ExchangeRate {
    pub from: Currency,
    pub to: Currency,
    rate: Decimal,
}

impl ExchangeRate {
    pub fn new(from: Currency, to: Currency, rate: Decimal) -> Result<ExchangeRate, MoneyError> {
        if from == to {
            return Err(MoneyError::InvalidCurrency);
        }
        Ok(ExchangeRate { from, to, rate })
    }

    /// Converts a Money from one Currency to another using the exchange rate.
    pub fn convert(&self, amount: &Money) -> Result<Money, MoneyError> {
        if amount.currency() != &self.from {
            return Err(MoneyError::InvalidCurrency);
        }
        let converted_amount = amount.amount() * self.rate;
        // Clone self.to as Money::from_decimal takes Currency by value
        Ok(Money::from_decimal(converted_amount, self.to.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find_currency;
    use rust_decimal_macros::*;
    use crate::currencies::iso::{EUR, GBP, USD};

    #[test]
    fn exchange_stores_rates() {
        let usd = find_currency("USD").unwrap();
        let eur = find_currency("EUR").unwrap();
        let gbp = find_currency("GBP").unwrap();

        // Clone the currency references when calling ExchangeRate::new
        let eur_usd_rate = ExchangeRate::new(usd.clone(), eur.clone(), dec!(1.5)).unwrap();
        let eur_gbp_rate = ExchangeRate::new(usd.clone(), gbp.clone(), dec!(1.6)).unwrap();

        let mut exchange = Exchange::new();
        exchange.set_rate(&eur_usd_rate);
        exchange.set_rate(&eur_gbp_rate);

        let fetched_rate = exchange.get_rate(&usd, &eur).unwrap();
        assert_eq!(fetched_rate.rate, dec!(1.5));

        let fetched_rate = exchange.get_rate(&usd, &gbp).unwrap();
        assert_eq!(fetched_rate.rate, dec!(1.6));
    }

    #[test]
    fn rate_convert() {
        let rate = ExchangeRate::new(USD, EUR, dec!(1.5)).unwrap();
        let amount = Money::from_minor(1_000, USD);
        let expected_amount = Money::from_minor(1_500, EUR);
        let converted_rate = rate.convert(&amount).unwrap();
        assert_eq!(converted_rate, expected_amount);
    }

    #[test]
    fn rate_convert_errors_if_currencies_do_not_match() {
        let rate = ExchangeRate::new(GBP, EUR, dec!(1.5)).unwrap();
        let amount = Money::from_minor(1_000, USD);

        assert_eq!(
            rate.convert(&amount).unwrap_err(),
            MoneyError::InvalidCurrency,
        );
    }

    #[test]
    fn rate_new_errors_if_currencies_are_equal() {
        let rate = ExchangeRate::new(GBP, GBP, dec!(1.5));
        assert_eq!(rate.unwrap_err(), MoneyError::InvalidCurrency,);
    }
}
