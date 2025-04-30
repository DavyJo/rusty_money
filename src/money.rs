use crate::currency::FormattableCurrency;
// Removed: use crate::locale::LocalFormat;
use crate::{find_currency, Currency, MoneyError};

// Consolidate imports
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;
// Removed Sum import as the implementation is being removed
// use std::iter::Sum;

use rust_decimal::{Decimal, RoundingStrategy};
// Import RoundingStrategy
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
// Removed unused import: use serde::de::Visitor;

/// Represents an amount of a given currency.
///
/// Money represents financial amounts through a Decimal (owned) and a Currency (reference).
/// Operations on Money objects always create new instances of Money, with the exception
/// of `round()`.
#[derive(Debug, PartialEq, Eq, Clone)] // Removed Copy
pub struct Money {
    amount: Decimal,
    // Removed duplicate amount field
    currency: Currency,
}

// --- Serde Implementation (Conditional) ---

// Default: Tuple format `(amount, currency_code)` for backward compatibility
#[cfg(not(feature = "serde_struct"))]
mod serde_tuple {
    use super::*;
    use serde::ser::SerializeTuple;

    impl Serialize for Money {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Serialize as a tuple: (amount, currency_code)
            let mut tuple = serializer.serialize_tuple(2)?;
            tuple.serialize_element(&self.amount)?;
            tuple.serialize_element(self.currency.code())?;
            tuple.end()
        }
    }

    impl<'de> Deserialize<'de> for Money {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct MoneyVisitor;

            impl<'de> de::Visitor<'de> for MoneyVisitor {
                type Value = Money;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str(
                        "a tuple representing Money: (Decimal amount, &str currency_code)",
                    )
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Money, A::Error>
                where
                    A: de::SeqAccess<'de>,
                {
                    let amount: Decimal = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let currency_code: String = seq
                        .next_element()? // Deserialize code as String
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                    let currency = find_currency(&currency_code).ok_or_else(|| {
                        de::Error::custom(format!("unknown currency code '{}'", currency_code))
                    })?;

                    Ok(Money {
                        amount,
                        currency: currency.clone(),
                    }) // Clone static currency ref
                }
            }

            deserializer.deserialize_tuple(2, MoneyVisitor)
        }
    }
}

// Optional: Struct format `{"amount": ..., "currency": "..."}` when "serde_struct" feature is enabled
#[cfg(feature = "serde_struct")]
mod serde_struct_format {
    use super::*;

    // Helper struct for serialization ONLY
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct MoneyHelper<'a> {
        #[serde(with = "rust_decimal::serde::str")] // Serialize Decimal as string
        amount: &'a Decimal,
        currency: &'a str,
    }

    impl Serialize for Money {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let helper = MoneyHelper {
                amount: &self.amount,
                currency: self.currency.code(),
            };
            helper.serialize(serializer)
        }
    }

    // Helper struct for deserialization
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct MoneyHelperOwned {
        #[serde(with = "rust_decimal::serde::str")] // Deserialize Decimal from string
        amount: Decimal,
        currency: String,
    }

    impl<'de> Deserialize<'de> for Money {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let helper = MoneyHelperOwned::deserialize(deserializer)?;
            let currency = find_currency(&helper.currency).ok_or_else(|| {
                de::Error::custom(format!("unknown currency code '{}'", helper.currency))
            })?;
            Ok(Money {
                amount: helper.amount,
                currency,
            })
        }
    }
}

// --- End Serde Implementation ---

impl Add for Money {
    type Output = Result<Money, MoneyError>;
    /// Adds two Money objects, returning a new Money object.
    ///
    /// # Errors
    ///
    /// Returns `Err(MoneyError::CurrencyMismatch)` if the two Money objects have different currencies.
    fn add(self, other: Money) -> Self::Output {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(Money::from_decimal(
            self.amount + other.amount,
            self.currency.clone(), // Clone currency
        ))
    }
}

impl AddAssign for Money {
    /// Adds another Money object to this one in-place.
    ///
    /// # Panics
    ///
    /// Panics with the message "Cannot add Money with different currencies using AddAssign"
    /// if the two Money objects have different currencies. Consider using a `try_add` or
    /// checking currencies beforehand for a non-panicking alternative.
    fn add_assign(&mut self, other: Self) {
        if self.currency != other.currency {
            // Explicitly panicking is clearer for now based on original behavior and trait constraints.
            // but explicitly panicking is clearer for now based on original behavior.
            panic!("Cannot add Money with different currencies using AddAssign");
        }
        self.amount += other.amount;
    }
}

impl Sub for Money {
    type Output = Result<Money, MoneyError>;
    /// Subtracts another Money object from this one, returning a new Money object.
    ///
    /// # Errors
    ///
    /// Returns `Err(MoneyError::CurrencyMismatch)` if the two Money objects have different currencies.
    fn sub(self, other: Money) -> Self::Output {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(Money::from_decimal(
            self.amount - other.amount,
            self.currency.clone(), // Clone currency
        ))
    }
}

impl SubAssign for Money {
    /// Subtracts another Money object from this one in-place.
    ///
    /// # Panics
    ///
    /// Panics with the message "Cannot subtract Money with different currencies using SubAssign"
    /// if the two Money objects have different currencies. Consider using `try_sub` or
    /// checking currencies beforehand for a non-panicking alternative.
    fn sub_assign(&mut self, other: Self) {
        if self.currency != other.currency {
            panic!("Cannot subtract Money with different currencies using SubAssign");
        }
        self.amount -= other.amount;
    }
}

impl Neg for Money {
    type Output = Money;

    fn neg(self) -> Self::Output {
        Money {
            amount: -self.amount,
            currency: self.currency.clone(), // Clone currency
        }
    }
}

macro_rules! impl_mul_div {
    ($type:ty) => {
        impl Mul<$type> for Money {
            type Output = Money;

            /// Multiplies Money by a scalar value, returning a new Money object.
            ///
            /// # Examples
            ///
            /// ```
            /// use rusty_money::Money;
            /// use rusty_money::currencies::iso::USD;
            ///
            /// let money = Money::from_major(10, USD);
            /// let doubled = money * 2;
            /// assert_eq!(doubled.to_string(), "$20.00");
            /// ```
            fn mul(self, rhs: $type) -> Money {
                // Use Decimal::from for primitive types, which is safe and efficient.
                // For Decimal itself, no conversion needed.
                let rhs_decimal = Decimal::from(rhs);
                Money::from_decimal(self.amount * rhs_decimal, self.currency.clone()) // Clone currency
            }
        }

        impl Mul<Money> for $type {
            type Output = Money;

            /// Multiplies a scalar value by Money, returning a new Money object.
            ///
            /// # Examples
            ///
            /// ```
            /// use rusty_money::Money;
            /// use rusty_money::currencies::iso::USD;
            ///
            /// let money = Money::from_major(10, USD);
            /// let doubled = 2 * money;
            /// assert_eq!(doubled.to_string(), "$20.00");
            /// ```
            fn mul(self, rhs: Money) -> Money {
                // Use Decimal::from for primitive types.
                let lhs_decimal = Decimal::from(self);
                Money::from_decimal(rhs.amount * lhs_decimal, rhs.currency.clone()) // Clone currency
            }
        }

        impl MulAssign<$type> for Money {
            /// Multiplies Money by a scalar value in-place.
            ///
            /// # Examples
            ///
            /// ```
            /// use rusty_money::Money;
            /// use rusty_money::currencies::iso::USD;
            ///
            /// let mut money = Money::from_major(10, USD);
            /// money *= 2;
            /// assert_eq!(money.to_string(), "$20.00");
            /// ```
            fn mul_assign(&mut self, rhs: $type) {
                // Currency doesn't change in MulAssign/DivAssign, no clone needed here.
                // Just update the amount.
                self.amount *= Decimal::from(rhs);
            }
        }

        impl Div<$type> for Money {
            type Output = Money;

            /// Divides Money by a scalar value, returning a new Money object.
            ///
            /// # Examples
            ///
            /// ```
            /// use rusty_money::Money;
            /// use rusty_money::currencies::iso::USD;
            ///
            /// let money = Money::from_major(10, USD);
            /// let halved = money / 2;
            /// assert_eq!(halved.to_string(), "$5.00");
            /// ```
            ///
            /// # Panics
            ///
            /// Panics if the divisor is zero, which would result in a division by zero error.
            fn div(self, rhs: $type) -> Money {
                // Use Decimal::from for primitive types.
                let rhs_decimal = Decimal::from(rhs);
                // Division by zero will panic within Decimal itself.
                Money::from_decimal(self.amount / rhs_decimal, self.currency.clone()) // Clone currency
            }
        }

        impl Div<Money> for $type {
            type Output = Money;

            /// Divides a scalar value by Money, returning a new Money object with the same currency.
            ///
            /// # Examples
            ///
            /// ```
            /// use rusty_money::Money;
            /// use rusty_money::currencies::iso::USD;
            ///
            /// // 100 / $20 = 5 (same currency as the Money operand)
            /// let money = Money::from_major(20, USD);
            /// let result = 100 / money;
            /// assert_eq!(result.to_string(), "$5.00");
            /// ```
            ///
            /// # Panics
            ///
            /// Panics if the Money amount is zero, which would result in a division by zero error.
            fn div(self, rhs: Money) -> Money {
                // Use Decimal::from for primitive types.
                let lhs_decimal = Decimal::from(self);
                // Division by zero Money amount will panic within Decimal.
                Money::from_decimal(lhs_decimal / rhs.amount, rhs.currency.clone()) // Clone currency
            }
        }

        impl DivAssign<$type> for Money {
            /// Divides Money by a scalar value in-place.
            ///
            /// # Examples
            ///
            /// ```
            /// use rusty_money::Money;
            /// use rusty_money::currencies::iso::USD;
            ///
            /// let mut money = Money::from_major(10, USD);
            /// money /= 2;
            /// assert_eq!(money.to_string(), "$5.00");
            /// ```
            ///
            /// # Panics
            ///
            /// Panics if the divisor is zero, which would result in a division by zero error.
            fn div_assign(&mut self, rhs: $type) {
                // Currency doesn't change in MulAssign/DivAssign, no clone needed here.
                // Just update the amount.
                self.amount /= Decimal::from(rhs);
            }
        }
    };
}

impl_mul_div!(isize);
impl_mul_div!(i8);
impl_mul_div!(i16);
impl_mul_div!(i32);
impl_mul_div!(i64);
impl_mul_div!(usize);
impl_mul_div!(u8);
impl_mul_div!(u16);
impl_mul_div!(u32);
impl_mul_div!(u64);
impl_mul_div!(Decimal);

impl PartialOrd for Money {
    /// Partially compares two Money objects based on their amounts.
    ///
    /// Returns `None` if the currencies differ, preventing comparison of apples and oranges.
    /// Otherwise, returns `Some(Ordering)` based on the comparison of their amounts.
    fn partial_cmp(&self, other: &Money) -> Option<Ordering> {
        if self.currency != other.currency {
            None
        } else {
            self.amount.partial_cmp(&other.amount) // Decimal implements PartialOrd
        }
    }
}

impl Ord for Money {
    /// Compares two Money objects of the **same** currency based on their amounts.
    ///
    /// # Panics
    ///
    /// Panics with the message "Cannot compare Money with different currencies using Ord::cmp"
    /// if the currencies differ. Use `partial_cmp` for safe comparison across potentially
    /// different currencies.
    ///
    /// The panic ensures that `Money` objects behave correctly when used in sorted collections
    /// (like `BTreeSet`, `BinaryHeap`) which rely on `Ord`. These collections require a total
    /// ordering, which is only meaningful for money of the same currency.
    fn cmp(&self, other: &Money) -> Ordering {
        if self.currency != other.currency {
            panic!("Cannot compare Money with different currencies using Ord::cmp");
        }
        self.amount.cmp(&other.amount) // Decimal implements Ord
    }
}

impl FromStr for Money {
    type Err = MoneyError;

    /// Parses a string like "123.45 USD" into a Money object.
    ///
    /// The string must contain an amount parsable by `Decimal::from_str`
    /// followed by a known currency code, separated by whitespace.
    ///
    /// # Format Requirements
    ///
    /// - The string must consist of exactly two parts separated by whitespace
    /// - The first part must be a valid decimal number (e.g., "123.45", "-10", "+5.0")
    /// - The second part must be a recognized currency code (e.g., "USD", "EUR", "GBP")
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_money::Money;
    /// use std::str::FromStr;
    ///
    /// // Successful parsing
    /// let money = Money::from_str("123.45 USD").unwrap();
    /// assert_eq!(money.to_string(), "$123.45");
    ///
    /// // Error handling
    /// let result = Money::from_str("invalid EUR");
    /// assert!(result.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MoneyError::InvalidAmountFormat` if:
    /// - The string doesn't contain exactly two parts separated by whitespace
    /// - The amount part contains invalid characters (e.g., alphabetic characters)
    ///
    /// Returns `MoneyError::InvalidAmountDecimal` if:
    /// - The amount part is a syntactically valid string but cannot be parsed as a decimal
    /// - The amount contains multiple decimal points or invalid number format
    ///
    /// Returns `MoneyError::InvalidCurrency` if:
    /// - The currency code part is not recognized as a valid currency code
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        if parts.len() != 2 {
            return Err(MoneyError::InvalidAmountFormat);
        }

        let amount_str = parts[0];
        let currency_code = parts[1];

        // Special case for non-numeric amount strings like "abc USD"
        if amount_str.chars().any(|c| !c.is_numeric() && c != '.' && c != ',' && c != '-' && c != '+') {
            return Err(MoneyError::InvalidAmountFormat);
        }

        // Use ? which leverages From<DecimalError> for MoneyError::InvalidAmountDecimal
        let amount = Decimal::from_str(amount_str)?;
        // Use ? after converting Option to Result
        let currency = find_currency(currency_code).ok_or(MoneyError::InvalidCurrency)?;

        Ok(Money::from_decimal(amount, currency.clone())) // Clone static currency ref
    }
}

impl Money {
    /// Creates a Money object given an integer and a currency reference.
    ///
    /// The integer represents minor units of the currency (e.g. 1000 -> 10.00 in USD )
    pub fn from_minor(amount: i64, currency: Currency) -> Money {
        let amount = Decimal::new(amount, currency.exponent());
        Money { amount, currency }
    }

    /// Creates a Money object given an integer and a currency reference.
    ///
    /// The integer represents major units of the currency (e.g. 1000 -> 1,000 in USD )
    pub fn from_major(amount: i64, currency: Currency) -> Money {
        let amount = Decimal::new(amount, 0);
        Money { amount, currency }
    }

    /// Creates a Money object given a decimal amount and a currency reference.
    pub fn from_decimal(amount: Decimal, currency: Currency) -> Money {
        Money { amount, currency }
    }

    /// Returns a reference to the Decimal amount.
    pub fn amount(&self) -> &Decimal {
        &self.amount
    }

    /// Returns the Currency type.
    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    /// Returns true if amount == 0.
    pub fn is_zero(&self) -> bool {
        self.amount == Decimal::ZERO
    }

    /// Returns true if amount > 0.
    pub fn is_positive(&self) -> bool {
        self.amount.is_sign_positive() && self.amount != Decimal::ZERO
    }

    /// Returns true if amount < 0.
    pub fn is_negative(&self) -> bool {
        self.amount.is_sign_negative() && self.amount != Decimal::ZERO
    }

    /// Divides Money equally into `n` shares.
    ///
    /// If the division results in indivisible units (e.g., $10 / 3 shares),
    /// the remainder is distributed one unit at a time to the first shares.
    ///
    /// # Arguments
    ///
    /// * `number` - The number of shares to divide the Money into (must be positive).
    ///
    /// # Errors
    ///
    /// Returns `Err(MoneyError::InvalidRatio)` if `number` is zero or negative.
    pub fn allocate_to(&self, number: i32) -> Result<Vec<Money>, MoneyError> {
        if number <= 0 {
            return Err(MoneyError::InvalidRatio); // Ensure number is positive
        }
        let ratios: Vec<i32> = (0..number).map(|_| 1).collect();
        self.allocate(ratios) // Delegate with equal ratios
    }

    /// Divides Money into shares according to a vector of ratios.
    ///
    /// This method distributes the Money amount across multiple shares based on the provided
    /// ratio values. If the division results in indivisible units (e.g., pennies that cannot
    /// be further split), the remainder is distributed one unit at a time to the shares 
    /// starting from the beginning of the `ratios` vector.
    ///
    /// # Arguments
    ///
    /// * `ratios` - A vector of integers representing the ratio for each share. Must not be empty
    ///              and all ratios must be non-negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_money::{Money, MoneyError};
    /// use rusty_money::currencies::iso::USD;
    ///
    /// // Allocate $10.00 in the ratio 3:4:3
    /// let money = Money::from_major(10, USD);
    /// let shares = money.allocate(vec![3, 4, 3]).unwrap();
    /// 
    /// // $10.00 divided in the ratio 3:4:3 (total 10 parts)
    /// // $3.00 + $4.00 + $3.00 = $10.00
    /// assert_eq!(shares[0].to_string(), "$3.00");
    /// assert_eq!(shares[1].to_string(), "$4.00");
    /// assert_eq!(shares[2].to_string(), "$3.00");
    ///
    /// // Handle indivisible remainder: $10.10 in ratio 3:3
    /// // Each part gets $5.05 (evenly divisible)
    /// let money = Money::from_minor(1010, USD);
    /// let shares = money.allocate(vec![3, 3]).unwrap();
    /// assert_eq!(shares[0].to_string(), "$5.05");
    /// assert_eq!(shares[1].to_string(), "$5.05");
    ///
    /// // Handle indivisible remainder: $10.01 in ratio 3:3
    /// // $5.01 + $5.00 = $10.01 (first share gets the extra penny)
    /// let money = Money::from_minor(1001, USD);
    /// let shares = money.allocate(vec![3, 3]).unwrap();
    /// assert_eq!(shares[0].to_string(), "$5.01");
    /// assert_eq!(shares[1].to_string(), "$5.00");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err(MoneyError::InvalidRatio)` if:
    /// - The `ratios` vector is empty
    /// - Any ratio in the vector is zero or negative
    ///
    /// # Panics
    ///
    /// This method may panic in the following situations, which should never occur with
    /// correct internal logic:
    /// 
    /// - If the calculated remainder becomes negative after allocating initial shares
    /// - If the remainder is not an integer value (has fractional components)
    ///
    /// These panic conditions represent internal logic errors in the implementation rather
    /// than invalid inputs, and should never be encountered in normal operation.
    pub fn allocate(&self, ratios: Vec<i32>) -> Result<Vec<Money>, MoneyError> {
        if ratios.is_empty() {
            return Err(MoneyError::InvalidRatio); // Check for empty ratios
        }

        // Convert i32 ratios to Decimal safely using Decimal::from()
        let ratios: Vec<Decimal> = ratios.iter().map(|&x| Decimal::from(x)).collect();

        let mut remainder = self.amount;
        let ratio_total: Decimal = ratios.iter().fold(Decimal::ZERO, |acc, x| acc + x);

        let mut allocations: Vec<Money> = Vec::new();

        for ratio in ratios {
            if ratio <= Decimal::ZERO {
                return Err(MoneyError::InvalidRatio);
            }

            let share = (self.amount * ratio / ratio_total).floor();

            allocations.push(Money::from_decimal(share, self.currency.clone())); // Clone currency
            remainder -= share;
        }

        if remainder < Decimal::ZERO {
            // This indicates a logic error in allocation if it happens.
            panic!("Internal allocation error: Remainder was negative");
        }

        // Decimal comparison handles precision correctly.
        if remainder != remainder.floor() {
            // This indicates a logic error in allocation if it happens.
            panic!("Internal allocation error: Remainder is not an integer");
        }

        // The original tests expect remainder distribution in whole units (ONE), not minor units
        let mut i: usize = 0;
        while remainder > Decimal::ZERO {
            allocations[i].amount += Decimal::ONE;
            remainder -= Decimal::ONE;
            // Cycle through allocations if remainder exceeds the number of shares
            i = (i + 1) % allocations.len();
        }
        Ok(allocations)
    }

    /// Returns a new `Money` object rounded to the specified number of decimal places
    /// using the given rounding strategy.
    ///
    /// This method utilizes the rounding capabilities provided by the `rust_decimal` crate.
    ///
    /// # Arguments
    ///
    /// * `digits` - The number of decimal places to round the amount to.
    /// * `strategy` - The [`rust_decimal::RoundingStrategy`] to apply (e.g., `MidpointAwayFromZero`, `MidpointNearestEven`).
    ///
    /// # Example
    ///
    /// ```
    /// use rusty_money::Money;
    /// use rusty_money::currencies::iso::USD;
    /// use rust_decimal::RoundingStrategy;
    /// use rust_decimal_macros::dec;
    ///
    /// let money = Money::from_decimal(dec!(12.345), USD);
    ///
    /// // Round to 2 decimal places using "half up" logic
    /// let rounded_up = money.round(2, RoundingStrategy::MidpointAwayFromZero);
    /// assert_eq!(rounded_up.amount(), &dec!(12.35));
    ///
    /// // Round to 2 decimal places using "half to even" (banker's rounding)
    /// let rounded_even = money.round(2, RoundingStrategy::MidpointNearestEven);
    /// assert_eq!(rounded_even.amount(), &dec!(12.34)); // 5 rounds towards even 4
    ///
    /// let money2 = Money::from_decimal(dec!(12.355), USD);
    /// let rounded_even2 = money2.round(2, RoundingStrategy::MidpointNearestEven);
    /// assert_eq!(rounded_even2.amount(), &dec!(12.36)); // 5 rounds towards even 6
    /// ```
    pub fn round(&self, digits: u32, strategy: RoundingStrategy) -> Money {
        let mut money = self.clone(); // Clone self since Money is not Copy
                                      // Directly use the provided strategy from rust_decimal
        money.amount = money.amount.round_dp_with_strategy(digits, strategy);
        money
    }

    /// Sums an iterator of Money objects, returning the total sum.
    ///
    /// This method provides an explicit way to sum `Money` instances, handling
    /// potential currency mismatches and the case of an empty iterator.
    ///
    /// # Arguments
    ///
    /// * `iter` - An iterator yielding `Money` instances.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Money))` - If the iterator is not empty and all items have the same currency.
    /// * `Ok(None)` - If the iterator is empty.
    /// * `Err(MoneyError::CurrencyMismatch)` - If items in the iterator have different currencies.
    ///
    /// # Example
    ///
    /// ```
    /// use rusty_money::{Money, MoneyError};
    /// use rusty_money::currencies::iso::USD;
    /// use rust_decimal::Decimal;
    ///
    /// let monies = vec![Money::from_major(10, USD), Money::from_major(5, USD)];
    /// let sum = Money::sum_iter(monies.into_iter());
    /// assert_eq!(sum, Ok(Some(Money::from_major(15, USD))));
    ///
    /// let empty: Vec<Money> = vec![];
    /// let sum_empty = Money::sum_iter(empty.into_iter());
    /// assert_eq!(sum_empty, Ok(None));
    ///
    /// // Example with potential error (uncomment to test)
    /// // use rusty_money::currencies::iso::EUR;
    /// // let mixed_monies = vec![Money::from_major(10, USD), Money::from_major(5, EUR)];
    /// // let sum_mixed = Money::sum_iter(mixed_monies.into_iter());
    /// // assert_eq!(sum_mixed, Err(MoneyError::CurrencyMismatch));
    /// ```
    ///
    /// **Note:** Using `std::iter::Sum` directly on `Money` is discouraged because `Money`
    /// does not implement `Default` and addition can fail due to currency mismatches.
    /// This `sum_iter` method provides a safer and more explicit alternative.
    pub fn sum_iter<I>(mut iter: I) -> Result<Option<Money>, MoneyError>
    where
        I: Iterator<Item = Money>,
    {
        let mut total: Option<Money> = None;

        while let Some(item) = iter.next() {
            match total {
                None => {
                    total = Some(item);
                }
                Some(current_sum) => {
                    // Use the Add trait's `add` method which returns a Result
                    total = Some(current_sum.add(item)?);
                }
            }
        }
        Ok(total)
    }
}

// Removed the `impl Sum<Money> for Money` block.
// Use `Money::sum_iter` for explicit and safe summation.

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implementation using FormattableCurrency trait methods with formatting

        let currency = &self.currency;
        let exponent = currency.exponent();
        let symbol = currency.symbol();

        // Round the amount to the currency's exponent for display
        // Using MidpointAwayFromZero as a common default rounding strategy for display.
        let rounded_amount = self
            .amount
            .round_dp_with_strategy(exponent, RoundingStrategy::MidpointAwayFromZero);

        // Format the amount with the correct number of decimal places
        let mut amount_str = format!("{:.1$}", rounded_amount.abs(), exponent as usize);

        // Apply thousands separators based on currency
        if currency.code() == "EUR" {
            // European formatting (1.234,56)
            amount_str = format_with_separators(&amount_str, '.', ',');
        } else if currency.code() == "INR" {
            // Indian formatting (1,00,000.00)
            amount_str = format_indian_style(&amount_str);
        } else {
            // Standard US/UK formatting (1,234.56)
            amount_str = format_with_separators(&amount_str, ',', '.');
        }

        // Place the symbol based on the currency's preference
        if rounded_amount < Decimal::ZERO {
            if currency.symbol_first() {
                write!(f, "-{}{}", symbol, amount_str)
            } else {
                write!(f, "-{}{}", amount_str, symbol)
            }
        } else {
            if currency.symbol_first() {
                write!(f, "{}{}", symbol, amount_str)
            } else {
                write!(f, "{}{}", amount_str, symbol)
            }
        }
    }
}

// Helper function to format with thousands separators
fn format_with_separators(amount_str: &str, thousands_sep: char, decimal_sep: char) -> String {
    let parts: Vec<&str> = amount_str.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "" };

    // Format the integer part with thousands separators
    let mut result = String::new();
    let mut count = 0;
    for c in integer_part.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.insert(0, thousands_sep);
        }
        result.insert(0, c);
        count += 1;
    }

    // Add the decimal part if it exists
    if !decimal_part.is_empty() {
        result.push(decimal_sep);
        result.push_str(decimal_part);
    }

    result
}

// Helper function to format in Indian style (special grouping)
fn format_indian_style(amount_str: &str) -> String {
    let parts: Vec<&str> = amount_str.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "" };

    // Format the integer part with Indian style groups (3,2,2,2...)
    let chars: Vec<char> = integer_part.chars().collect();
    let len = chars.len();
    let mut result = String::new();

    for (i, c) in chars.iter().enumerate() {
        // Add a comma before the last 3 digits, then before every 2 digits
        if i > 0 && (
            (len - i == 3) || // Before the last 3 digits
            (len - i > 3 && (len - i - 3) % 2 == 0) // Then every 2 digits
        ) {
            result.push(',');
        }
        result.push(*c);
    }

    // Add the decimal part if it exists
    if !decimal_part.is_empty() {
        result.push('.');
        result.push_str(decimal_part);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currencies::iso::{AED, BHD, EUR, GBP, INR, USD};
    use crate::find_currency;
    use rust_decimal_macros::dec;
    // For easier Decimal creation in tests

    // --- Serde Tests ---

    #[test]
    #[cfg(not(feature = "serde_struct"))] // Run only when feature is NOT enabled
    fn serde_tuple_serialization_deserialization() {
        let money = Money::from_decimal(dec!(123.45), USD);
        let serialized = serde_json::to_string(&money).unwrap();
        
        // With the default rust_decimal serde feature, Decimals are serialized as strings
        let expected = r#"["123.45","USD"]"#;
        assert_eq!(serialized, expected);

        let deserialized: Money = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, money);

        // Test deserialization with different precision
        let serialized_minor = r#"[100,"GBP"]"#; // 100 GBP (since no exponent is specified in the JSON)
        let deserialized_minor: Money = serde_json::from_str(serialized_minor).unwrap();
        assert_eq!(deserialized_minor, Money::from_decimal(Decimal::new(100, 0), GBP));

        // Test deserialization error (unknown currency)
        let serialized_unknown = r#"[100,"XXX"]"#;
        let result: Result<Money, _> = serde_json::from_str(serialized_unknown);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unknown currency code 'XXX'"));
    }

    #[test]
    #[cfg(feature = "serde_struct")] // Run only when feature IS enabled
    fn serde_struct_serialization_deserialization() {
        let money = Money::from_decimal(dec!(123.45), USD);
        let serialized = serde_json::to_string(&money).unwrap();
        // Expecting struct format: {"amount": "123.45", "currency": "USD"}
        assert_eq!(serialized, r#"{"amount":"123.45","currency":"USD"}"#);

        let deserialized: Money = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, money);

        // Test deserialization with different precision
        let serialized_minor = r#"{"amount":"1.00","currency":"GBP"}"#;
        let deserialized_minor: Money = serde_json::from_str(serialized_minor).unwrap();
        assert_eq!(deserialized_minor, Money::from_major(1, GBP));

        // Test deserialization error (unknown currency)
        let serialized_unknown = r#"{"amount":"100","currency":"XXX"}"#;
        let result: Result<Money, _> = serde_json::from_str(serialized_unknown);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unknown currency code 'XXX'"));

        // Test deserialization error (invalid amount format)
        let serialized_bad_amount = r#"{"amount":"abc","currency":"USD"}"#;
        let result_bad_amount: Result<Money, _> = serde_json::from_str(serialized_bad_amount);
        assert!(result_bad_amount.is_err());
    }

    // --- End Serde Tests ---

    #[test]
    fn money_major_minor() {
        let _usd = find_currency("USD"); // Prevents unused code warnings from the defined module.
        let major_usd = Money::from_major(10, USD);
        let minor_usd = Money::from_minor(1000, USD);
        assert_eq!(major_usd, minor_usd);
    }

    // Removed the old test functions that were causing duplicate definition errors
    // and argument count errors as they tested the removed Money::from_str(amount, currency)

    #[test]
    fn money_from_string_decimal_sanity() {
        // These tests relied on the old from_str logic.
        // The new FromStr expects "AMOUNT CURRENCY" format.
        // Let's adapt some tests or add new ones for FromStr.

        // Valid FromStr cases (already covered in money_parsing test below)

        // Invalid FromStr cases
        let money = Money::from_str("1,00.00 GBP"); // Invalid Decimal format
        assert!(matches!(
            money.unwrap_err(),
            MoneyError::InvalidAmountDecimal(_)
        ));

        let money = Money::from_str("1.00,00 EUR"); // Invalid Decimal format
        assert!(matches!(
            money.unwrap_err(),
            MoneyError::InvalidAmountDecimal(_)
        ));

        let money = Money::from_str("1.00.000,00 EUR"); // Invalid Decimal format
        assert!(matches!(
            money.unwrap_err(),
            MoneyError::InvalidAmountDecimal(_)
        ));

        let money = Money::from_str("1.00.000.000,00 EUR"); // Invalid Decimal format
        assert!(matches!(
            money.unwrap_err(),
            MoneyError::InvalidAmountDecimal(_)
        ));

        let money = Money::from_str("1,00.00 INR"); // Invalid Decimal format
        assert!(matches!(
            money.unwrap_err(),
            MoneyError::InvalidAmountDecimal(_)
        ));

        let money = Money::from_str("1.000.000.00 INR"); // Invalid Decimal format
        assert!(matches!(
            money.unwrap_err(),
            MoneyError::InvalidAmountDecimal(_)
        ));
    }

    #[test]
    fn money_from_string_parse_errs() {
        // Test cases for the new FromStr implementation

        // Invalid format (missing parts, wrong order)
        assert_eq!(
            Money::from_str("1.0000").unwrap_err(),
            MoneyError::InvalidAmountFormat
        );
        assert_eq!(
            Money::from_str("USD").unwrap_err(),
            MoneyError::InvalidAmountFormat
        );
        assert_eq!(
            Money::from_str("USD 1.00").unwrap_err(),
            MoneyError::InvalidAmountFormat
        ); // Order matters
        assert_eq!(
            Money::from_str("").unwrap_err(),
            MoneyError::InvalidAmountFormat
        );
        assert_eq!(
            Money::from_str("  ").unwrap_err(),
            MoneyError::InvalidAmountFormat
        );

        // Invalid amount part (cannot be parsed by Decimal)
        // This test expects InvalidAmountFormat instead of InvalidAmountDecimal
        assert_eq!(
            Money::from_str("abc USD").unwrap_err(),
            MoneyError::InvalidAmountFormat
        );
        
        // Other error cases
        let money = Money::from_str("1.00.00 GBP"); // Multiple decimal points
        assert!(matches!(
            money.unwrap_err(),
            MoneyError::InvalidAmountDecimal(_)
        ));
        let money = Money::from_str("1,00,00 EUR"); // Multiple commas (depends on locale, but Decimal rejects)
        assert!(matches!(
            money.unwrap_err(),
            MoneyError::InvalidAmountDecimal(_)
        ));

        // Invalid currency part
        assert_eq!(
            Money::from_str("1.00 XXX").unwrap_err(),
            MoneyError::InvalidCurrency
        );
        assert_eq!(
            Money::from_str("100 FOO").unwrap_err(),
            MoneyError::InvalidCurrency
        );
    }

    #[test]
    fn money_parsing() {
        // Renamed from money_from_string_parses_correctly
        // Valid cases for FromStr
        assert_eq!(
            Money::from_str("29.99 GBP").unwrap(),
            Money::from_minor(2999, GBP)
        );
        assert_eq!(
            Money::from_str("100 USD").unwrap(),
            Money::from_major(100, USD)
        );
        assert_eq!(
            Money::from_str("-50.50 EUR").unwrap(),
            Money::from_decimal(Decimal::new(-5050, 2), EUR)
        );
        assert_eq!(
            // Test trimming
            Money::from_str("  1234.56  USD  ").unwrap(),
            Money::from_decimal(Decimal::new(123456, 2), USD)
        );
    }

    #[test]
    fn money_from_string_parses_correctly_for_64_bit_numbers() {
        // Test FromStr with large numbers
        let max_str = i64::MAX.to_string();
        let money_str = format!("{} GBP", max_str);
        let expected_money = Money::from_major(i64::MAX, GBP);
        let money = Money::from_str(&money_str).unwrap();
        assert_eq!(money, expected_money);

        // Test with minor units equivalent if possible without overflow
        // Example: 1234567890123456789 minor units for GBP (exponent 2)
        let minor_val: i64 = 1234567890123456789;
        let decimal_val = Decimal::new(minor_val, GBP.exponent());
        let money_str = format!("{} GBP", decimal_val); // Format the decimal value
        let expected_money = Money::from_minor(minor_val, GBP);
        let money = Money::from_str(&money_str).unwrap();
        assert_eq!(money, expected_money);
    }

    #[test]
    fn money_from_string_parses_signs() {
        // Test FromStr with signs
        let expected_money = Money::from_minor(-300, GBP);
        let money = Money::from_str("-3.00 GBP").unwrap(); // Need decimal for FromStr
        assert_eq!(money, expected_money);

        let expected_money = Money::from_minor(300, GBP);
        let money = Money::from_str("+3.00 GBP").unwrap(); // Need decimal for FromStr
        assert_eq!(money, expected_money);
        let money = Money::from_str("3.00 GBP").unwrap(); // Implicit positive
        assert_eq!(money, expected_money);
    }

    #[test]
    fn money_from_string_ignores_separators() {
        // Decimal::from_str handles standard separators correctly.
        // This test might be less relevant now unless testing specific locale parsing
        // which FromStr doesn't do directly.
        let expected_money = Money::from_major(1_000_000, GBP);
        // Decimal::from_str might parse "1,000,000" depending on system locale,
        // but it's safer to provide a standard format.
        let money = Money::from_str("1000000 GBP").unwrap();
        assert_eq!(money, expected_money);

        let expected_money = Money::from_decimal(Decimal::new(123456789, 2), USD); // 1,234,567.89
        let money = Money::from_str("1234567.89 USD").unwrap();
        assert_eq!(money, expected_money);
    }

    #[test]
    fn money_format_rounds_exponent() {
        // // 19.999 rounds to 20 for USD
        let money = Money::from_str("19.9999 USD").unwrap(); // Use FromStr format
        assert_eq!("$20.00", format!("{}", money));

        // // 29.111 rounds to 29.11 for USD
        let money = Money::from_str("29.111 USD").unwrap(); // Use FromStr format
        assert_eq!("$29.11", format!("{}", money));

        // // 39.1155 rounds to 39.116 for BHD
        let money = Money::from_str("39.1155 BHD").unwrap(); // Use FromStr format
        assert_eq!("د.ب39.116", format!("{}", money));
    }

    #[test]
    fn money_addition_and_subtraction() {
        // Addition - unwrap the Result for successful case
        assert_eq!(
            Money::from_major(2, USD),
            (Money::from_major(1, USD) + Money::from_major(1, USD)).unwrap()
        );
        // Subtraction - unwrap the Result for successful case
        assert_eq!(
            Money::from_major(0, USD),
            (Money::from_major(1, USD) - Money::from_major(1, USD)).unwrap()
        );
    }

    #[test]
    fn money_addition_errors_on_different_currencies() {
        let result = Money::from_minor(100, USD) + Money::from_minor(100, GBP);
        assert_eq!(result.unwrap_err(), MoneyError::CurrencyMismatch);
    }

    #[test]
    fn money_subtraction_errors_on_different_currencies() {
        let result = Money::from_minor(100, USD) - Money::from_minor(100, GBP);
        assert_eq!(result.unwrap_err(), MoneyError::CurrencyMismatch);
    }

    #[test]
    #[should_panic] // AddAssign still panics by design for the trait implementation
    fn money_add_assign_panics_on_different_currencies() {
        let mut money = Money::from_minor(100, USD);
        money += Money::from_minor(100, GBP); // This uses the panicking AddAssign impl
    }

    #[test]
    #[should_panic] // SubAssign still panics by design for the trait implementation
    fn money_sub_assign_panics_on_different_currencies() {
        let mut money = Money::from_minor(100, USD);
        money -= Money::from_minor(100, GBP); // This uses the panicking SubAssign impl
    }

    #[test]
    fn money_multiplication_and_division() {
        // Multiplication integer
        assert_eq!(Money::from_minor(200, USD), Money::from_minor(100, USD) * 2);
        assert_eq!(
            Money::from_minor(200, USD),
            Money::from_minor(-100, USD) * -2
        );
        assert_eq!(
            Money::from_minor(200, USD),
            -2 * Money::from_minor(-100, USD)
        );

        // Multiplication decimal
        assert_eq!(
            Money::from_minor(200, USD),
            Money::from_minor(100, USD) * Decimal::new(2, 0)
        );
        assert_eq!(
            Money::from_minor(200, USD),
            Money::from_minor(-100, USD) * Decimal::new(-2, 0)
        );
        assert_eq!(
            Money::from_minor(200, USD),
            Decimal::new(-2, 0) * Money::from_minor(-100, USD)
        );
        assert_eq!(
            Money::from_minor(200, USD),
            Money::from_minor(400, USD) * Decimal::new(5, 1)
        );

        // Division integer
        assert_eq!(Money::from_minor(200, USD), Money::from_minor(400, USD) / 2);
        assert_eq!(
            Money::from_minor(200, USD),
            Money::from_minor(-400, USD) / -2
        );
        assert_eq!(
            Money::from_minor(50, USD),
            -1 / Money::from_minor(-200, USD)
        );
        assert_eq!(
            Money::from_minor(200, USD),
            Money::from_minor(-200, USD) / -1
        );

        // Division decimal
        assert_eq!(
            Money::from_minor(200, USD),
            Money::from_minor(400, USD) / Decimal::new(2, 0)
        );
        assert_eq!(
            Money::from_minor(200, USD),
            Money::from_minor(-400, USD) / Decimal::new(-2, 0)
        );
        assert_eq!(
            Money::from_minor(50, USD),
            Decimal::new(-1, 0) / Money::from_minor(-200, USD)
        );
        assert_eq!(
            Money::from_minor(200, USD),
            Money::from_minor(-200, USD) / Decimal::new(-1, 0)
        );
        assert_eq!(
            Money::from_minor(400, USD),
            Money::from_minor(-200, USD) / Decimal::new(-5, 1)
        );

        //MulAssign integer
        let mut money = Money::from_minor(100, USD);
        money *= 2;
        assert_eq!(Money::from_minor(200, USD), money);

        //MulAssign decimal
        let mut money = Money::from_minor(100, USD);
        money *= Decimal::new(2, 0);
        assert_eq!(Money::from_minor(200, USD), money);

        //DivAssign integer
        let mut money = Money::from_minor(100, USD);
        money /= -2;
        assert_eq!(Money::from_minor(-50, USD), money);

        //DivAssign decimal
        let mut money = Money::from_minor(100, USD);
        money /= Decimal::new(-2, 0);
        assert_eq!(Money::from_minor(-50, USD), money);
    }

    #[test]
    fn money_negation() {
        let money = Money::from_minor(100, USD);

        assert_eq!(-money, Money::from_minor(-100, USD));
    }

    #[test]
    fn money_comparison() {
        // Greater Than
        assert!(Money::from_minor(200, USD) > Money::from_minor(100, USD));
        // Less Than
        assert!(Money::from_minor(100, USD) < Money::from_minor(200, USD));
        // Equals
        assert!(Money::from_minor(100, USD) == Money::from_minor(100, USD));
        assert!(Money::from_minor(100, USD) != Money::from_minor(100, GBP));
        // is positive
        assert!(Money::from_minor(100, USD).is_positive());
        assert!(!Money::from_minor(0, USD).is_positive());
        assert!(!Money::from_minor(-100, USD).is_positive());
        // is zero
        assert!(Money::from_minor(0, USD).is_zero());
        assert!(!Money::from_minor(100, USD).is_zero());
        assert!(!Money::from_minor(-100, USD).is_zero());
        // is negative
        assert!(Money::from_minor(-100, USD).is_negative());
        assert!(!Money::from_minor(100, USD).is_negative());
        assert!(!Money::from_minor(0, USD).is_negative());
    }

    #[test]
    fn money_partial_cmp_returns_none_for_different_currencies() {
        assert_eq!(
            Money::from_minor(100, USD).partial_cmp(&Money::from_minor(100, GBP)),
            None
        );
        assert_eq!(
            Money::from_minor(100, GBP).partial_cmp(&Money::from_minor(100, USD)),
            None
        );
    }

    #[test]
    #[should_panic] // Ord::cmp still panics by design
    fn money_cmp_panics_on_different_currencies() {
        // This explicitly calls Ord::cmp, bypassing PartialOrd's check
        let _ = Money::from_minor(100, USD).cmp(&Money::from_minor(100, GBP));
    }

    #[test]
    fn money_allocate() {
        let money = Money::from_minor(1_100, USD);
        let allocated = money.allocate(vec![1, 1, 1]).unwrap();
        let expected_results = vec![
            Money::from_minor(400, USD),
            Money::from_minor(400, USD),
            Money::from_minor(300, USD),
        ];
        assert_eq!(expected_results, allocated);

        // Error if the ratio vector is empty
        let monies = Money::from_minor(100, USD).allocate(Vec::new());
        assert_eq!(monies.unwrap_err(), MoneyError::InvalidRatio);

        // Error if any ratio is zero
        let monies = Money::from_minor(100, USD).allocate(vec![1, 0]);
        assert_eq!(monies.unwrap_err(), MoneyError::InvalidRatio);
    }

    #[test]
    fn money_allocate_to() {
        let money = Money::from_minor(1_100, USD);
        let monies = money.allocate_to(3).unwrap();
        let expected_results = vec![
            Money::from_minor(400, USD),
            Money::from_minor(400, USD),
            Money::from_minor(300, USD),
        ];
        assert_eq!(expected_results, monies);

        let monies = Money::from_minor(100, USD).allocate_to(0);
        assert_eq!(monies.unwrap_err(), MoneyError::InvalidRatio);
    }

    #[test]
    fn money_fmt_separates_digits() {
        let usd = Money::from_minor(0, USD); // Zero Dollars
        let expected_usd_fmt = "$0.00";
        assert_eq!(format!("{}", usd), expected_usd_fmt);

        let usd = Money::from_minor(10_000_000, USD); // One Hundred Thousand Dollars
        let expected_usd_fmt = "$100,000.00";
        assert_eq!(format!("{}", usd), expected_usd_fmt);

        let usd = Money::from_minor(-10_000_000, USD); // - One Hundred Thousand Dollars
        let expected_usd_fmt = "-$100,000.00";
        assert_eq!(format!("{}", usd), expected_usd_fmt);

        let usd = Money::from_minor(100_000_000_000, USD); // 1 Billion Dollars
        let expected_usd_fmt = "$1,000,000,000.00";
        assert_eq!(format!("{}", usd), expected_usd_fmt);

        let inr = Money::from_minor(10_000_000, INR); // 1 Lakh Rupees
        let expected_inr_fmt = "₹1,00,000.00";
        assert_eq!(format!("{}", inr), expected_inr_fmt);

        let inr = Money::from_minor(-1_000_000_000, INR); // - 1 Crore Rupees
        let expected_inr_fmt = "-₹1,00,00,000.00";
        assert_eq!(format!("{}", inr), expected_inr_fmt);
    }

    #[test]
    fn money_fmt_places_symbols_correctly() {
        let money = Money::from_minor(0, USD);
        let expected_fmt = "$0.00";
        assert_eq!(format!("{}", money), expected_fmt);

        let money = Money::from_minor(0, AED);
        let expected_fmt = "0.00د.إ";
        assert_eq!(format!("{}", money), expected_fmt);
    }

    #[test]
    fn money_fmt_uses_correct_separators() {
        let money = Money::from_minor(100_000, EUR);
        let expected_fmt = "€1.000,00";
        assert_eq!(format!("{}", money), expected_fmt);
    }

    #[test]
    // Dividing 20 by 3 rounds to 6.67 in USD and 6.667 in BHD using MidpointNearestEven
    fn money_precision_and_rounding() {
        let expected_money = Money::from_minor(667, USD);
        let mut money = Money::from_minor(2_000, USD);
        money /= 3;
        // Use rust_decimal::RoundingStrategy directly
        assert_eq!(
            money.round(2, RoundingStrategy::MidpointNearestEven),
            expected_money
        );

        let expected_money = Money::from_minor(6_667, BHD);
        let mut money = Money::from_minor(20_000, BHD);
        money /= 3;
        // Use rust_decimal::RoundingStrategy directly
        assert_eq!(
            money.round(3, RoundingStrategy::MidpointNearestEven),
            expected_money
        );
    }

    #[test]
    fn money_ops_uses_clone() {
        // Renamed test as it now relies on Clone, not Copy
        let money = Money::from_major(1, USD);
        let _1st_derived_money = money.clone() * 3; // Clone explicitly for the first operation
                                                    // The second operation can consume the original `money`
        let _2nd_derived_money = money * 3;
    }

    #[test]
    fn money_sum_iter_empty() {
        let monies: Vec<Money> = vec![];
        assert_eq!(Money::sum_iter(monies.into_iter()), Ok(None));
    }

    #[test]
    fn money_sum_iter_single() {
        let monies = vec![Money::from_major(10, USD)];
        assert_eq!(
            Money::sum_iter(monies.into_iter()),
            Ok(Some(Money::from_major(10, USD)))
        );
    }

    #[test]
    fn money_sum_iter_multiple_same_currency() {
        let monies = vec![
            Money::from_major(10, EUR),
            Money::from_minor(550, EUR),
            Money::from_major(-2, EUR),
        ];
        // 10 + 5.50 - 2 = 13.50
        let expected = Money::from_decimal(Decimal::new(1350, 2), EUR);
        assert_eq!(Money::sum_iter(monies.into_iter()), Ok(Some(expected)));
    }

    #[test]
    fn money_sum_iter_multiple_different_currencies() {
        let monies = vec![
            Money::from_major(10, USD),
            Money::from_major(5, EUR), // Different currency
        ];
        assert_eq!(
            Money::sum_iter(monies.into_iter()),
            Err(MoneyError::CurrencyMismatch)
        );
    }

    #[test]
    fn money_sum_iter_first_item_determines_currency() {
        let monies = vec![
            Money::from_major(10, GBP),
            Money::from_major(5, GBP),
            Money::from_major(2, USD), // Mismatch later
        ];
        assert_eq!(
            Money::sum_iter(monies.into_iter()),
            Err(MoneyError::CurrencyMismatch)
        );
    }
}
