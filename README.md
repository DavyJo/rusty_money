# rusty-money

[![Build Status]][Github Action]
[![Latest Version]][crates.io]
[![Docs]][docs.rs]

[Build Status]: https://github.com/varunsrin/rusty_money/actions/workflows/rust.yml/badge.svg
[Github Action]: https://github.com/varunsrin/rusty_money/actions/workflows/rust.yml
[Latest Version]: https://img.shields.io/crates/v/rusty-money.svg
[crates.io]: https://crates.io/crates/rusty-money
[Docs]: https://docs.rs/rusty-money/badge.svg
[docs.rs]: https://docs.rs/rusty-money

`rusty-money` handles the messy parts of dealing with money like rounding, precision, parsing and internationalization.
It supports [ISO-4217](https://en.wikipedia.org/wiki/ISO_4217) currencies, common crypto currencies and lets you
define your own. The main items exported by the library are `Money` and the `iso` and `crypto` currency sets.

## Usage

A `Money` object is created by supplying an amount and a currency. Amounts can be specified in numeric or string types
but will be stored as precise decimals internally.

## Features: Currency Sets

rusty_money provides two currency sets for convenience : `iso`, which implements ISO-4217 currencies and `crypto` which
implements popular cryptocurrencies. `iso` is enabled by default, and you can add `crypto` by enabling the feature:

```toml
[dependencies]
rusty-money = { version = "0.4.1", features = ["iso", "crypto"] }
```

The currency sets can then be used like this:

```rust
use rusty_money::Money;
use rusty_money::currencies::{iso, crypto};

Money::from_major(2_000, iso::USD);        // 2000 U.S Dollars
Money::from_major(2_000, iso::GBP);        // 2000 British Pounds
Money::from_major(2, crypto::BTC);         // 2 Bitcoin
```

Money objects of the same currency can be compared:

 ```rust
use rusty_money::{Money};
use rusty_money::currencies::iso::USD;
let hundred = Money::from_minor(10_000, USD);
let thousand = Money::from_minor(100_000, USD);

println!("{}", thousand > hundred);     // false
println!("{}", thousand.is_positive()); // true
```

## Precision, Rounding and Math

Money objects are immutable, and operations that change amounts create a new instance of Money. Amounts are stored
as 128 bit fixed-precision [Decimals](https://github.com/paupino/rust-decimal), and handle values as large as
$2^{96}$ / $10^{28}$. Operations on Money retain the maximum possible precision. When you want less
precision, you call the `round` function, which  supports three modes:

* [Half Up](https://en.wikipedia.org/wiki/Rounding#Round_half_up)
* [Half Down](https://en.wikipedia.org/wiki/Rounding#Round_half_down)
* [Half Even](https://en.wikipedia.org/wiki/Rounding#Round_half_even) (default)

Money can be added, subtracted, multiplied and divided like this:

```rust
use rusty_money::Money;
use rusty_money::currencies::iso;
use rust_decimal::RoundingStrategy; // Import the rounding strategy

// Addition and Subtraction now return Result
let sum = Money::from_minor(100, iso::USD) + Money::from_minor(100, iso::USD);  // Ok(2 USD)
assert_eq!(sum.unwrap(), Money::from_minor(200, iso::USD));
let diff = Money::from_minor(100, iso::USD) - Money::from_minor(100, iso::USD); // Ok(0 USD)
assert_eq!(diff.unwrap(), Money::from_minor(0, iso::USD));

// Multiplication and Division still return Money (panic on division by zero)
let product = Money::from_minor(100, iso::USD) * 3;                           // 3 USD
assert_eq!(product, Money::from_minor(300, iso::USD));
let quotient = Money::from_minor(100, iso::USD) / 3;                          // 0.333... USD
// Note: quotient amount is Decimal(33.333...) which might not be directly representable as minor units

// Use FromStr trait to parse a money string with amount and currency code
let usd = "2000.005 USD".parse::<Money>().unwrap();                           // 2000.005 USD
// Or with a negative amount - notice format is "AMOUNT CURRENCY"
let neg_usd = "-2000.005 USD".parse::<Money>().unwrap();                      // -2000.005 USD
// Use RoundingStrategy directly for rounding
let r1 = usd.round(2, RoundingStrategy::MidpointNearestEven);                 // 2000.00 USD
assert_eq!(r1, Money::from_minor(200000, iso::USD));
let r2 = usd.round(2, RoundingStrategy::MidpointAwayFromZero);                // 2000.01 USD
assert_eq!(r2, Money::from_minor(200001, iso::USD));
let r3 = usd.round(0, RoundingStrategy::MidpointAwayFromZero);                // 2000 USD
assert_eq!(r3, Money::from_minor(200000, iso::USD));
```

## Formatting

Calling `format!` or `println!` on Money returns a string with a rounded amount, using separators and symbols
according to the locale of the currency. If you need to customize this output, the `Formatter` module
accepts a more detailed set of parameters.

```rust
use rusty_money::Money;
use rusty_money::currencies::iso;
use std::str::FromStr;

let usd = "-2000.009 USD".parse::<Money>().unwrap();
let eur = "-2000.009 EUR".parse::<Money>().unwrap();

println!("{}", usd);                                        // -$2,000.01
println!("{}", eur);                                        // -€2.000,01
```

## Exchange

The library also provides two additional types - `Exchange` and `ExchangeRates` to convert Money from one currency
to another.

```rust
use rusty_money::{Money, Exchange, ExchangeRate};
use rusty_money::currencies::iso;
use rust_decimal_macros::*;

// Convert 1000 USD to EUR at a 2:1 exchange rate.
let rate = ExchangeRate::new(iso::USD, iso::EUR, dec!(0.5)).unwrap();
rate.convert(&Money::from_minor(100_000, iso::USD));                    // 500 EUR

// An Exchange can be used to store ExchangeRates for later use
let mut exchange = Exchange::new();
exchange.set_rate(&rate);
exchange.get_rate(&iso::USD, &iso::EUR);
```
