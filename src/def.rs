use crate::currency::FormattableCurrency;
use crate::{Locale, Money};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter::Sum;
use crate::cur::CurrencyCode::EUR;

/// Represents a single ISO-4217 currency (e.g. USD).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct Currency {
    pub code: &'static str,
    pub exponent: u32,
    pub locale: Locale,
    pub minor_units: u64,
    pub name: &'static str,
    pub symbol: &'static str,
    pub symbol_first: bool,
}

impl FormattableCurrency for Currency {
    fn to_string(&self) -> String {
        self.code().to_string()
    }

    fn exponent(&self) -> u32 {
        self.exponent
    }

    fn code(&self) -> &'static str {
        self.code
    }

    fn locale(&self) -> Locale {
        self.locale
    }

    fn symbol(&self) -> &'static str {
        self.symbol
    }

    fn symbol_first(&self) -> bool {
        self.symbol_first
    }
}

impl Sum for Money<Currency> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = None;
        for money in iter {
            if total.is_none() {
                total = Some(Money::from_major(0, money.currency()))
            }
            if let Some(total) = total.as_mut() {
                *total += money;
            }
        }
        if let Some(total) = total {
            total
        } else {
            Money::from_major(0, EUR.into())
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Money;
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use crate::cur::CurrencyCode::{BTC, USD};

    #[test]
    fn find_returns_known_currencies() {
        let currency_by_alpha = USD.as_currency();
        assert_eq!(currency_by_alpha.code, "USD");
        assert_eq!(currency_by_alpha.exponent, 2);
        assert_eq!(currency_by_alpha.symbol, "$");
    }

    #[test]
    fn currency_can_be_accessed_by_reference() {
        assert_eq!(USD.as_currency().code, "USD");
        assert_eq!(USD.as_currency().exponent, 2);
        assert_eq!(USD.as_currency().symbol, "$");
    }

    #[test]
    fn test_sum() {
        let money1 = Money::from_decimal(Decimal::from_str("10.1").unwrap(), USD.as_currency());
        let money2 = Money::from_decimal(Decimal::from_str("20.2").unwrap(), USD.as_currency());
        let money3 = Money::from_decimal(Decimal::from_str("30.3").unwrap(), USD.as_currency());

        let monies = vec![money1, money2, money3];

        let total: Money<Currency> = monies.into_iter().sum();

        assert_eq!(total.amount(), &Decimal::from_str("60.6").unwrap());
        assert_eq!(total.currency(), USD.as_currency());
    }

    #[test]
    #[should_panic]
    fn test_fail_sum() {
        let money1 = Money::from_decimal(Decimal::from_str("10.1").unwrap(), USD.as_currency());
        let money2 = Money::from_decimal(Decimal::from_str("20.2").unwrap(), USD.as_currency());
        let money3 = Money::from_decimal(Decimal::from_str("30.3").unwrap(), EUR.as_currency());

        let monies = vec![money1, money2, money3];

        let _total: Money<Currency> = monies.into_iter().sum();
    }

    #[test]
    fn find_returns_known_currencies_crypto() {
        let currency_by_code = BTC.as_currency();
        assert_eq!(currency_by_code.code, "BTC");
        assert_eq!(currency_by_code.exponent, 8);
        assert_eq!(currency_by_code.symbol, "₿");
    }
}
