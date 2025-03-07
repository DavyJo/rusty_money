use crate::Currency;
use crate::Locale::EnUs;

pub const BTC: &'static Currency = &Currency {
    exponent: 8,
    code: "BTC",
    locale: EnUs,
    minor_units: 100_000_000,
    name: "Bitcoin",
    symbol: "₿",
    symbol_first: true,
};
pub const COMP: &'static Currency = &Currency {
    exponent: 18,
    code: "COMP",
    locale: EnUs,
    minor_units: 1_000_000_000_000_000_000,
    name: "Compound",
    symbol: "COMP",
    symbol_first: false,
};
pub const DAI: &'static Currency = &Currency {
    exponent: 18,
    code: "DAI",
    locale: EnUs,
    minor_units: 1_000_000_000_000_000_000,
    name: "Dai Stablecoin",
    symbol: "DAI",
    symbol_first: false,
};
pub const ETH: &'static Currency = &Currency {
    exponent: 18,
    code: "ETH",
    locale: EnUs,
    minor_units: 1_000_000_000_000_000_000,
    name: "Ethereum",
    symbol: "ETH",
    symbol_first: false,
};
pub const MKR: &'static Currency = &Currency {
    exponent: 18,
    code: "MKR",
    locale: EnUs,
    minor_units: 1_000_000_000_000_000_000,
    name: "Maker",
    symbol: "MKR",
    symbol_first: false,
};
pub const UNI: &'static Currency = &Currency {
    exponent: 18,
    code: "UNI",
    locale: EnUs,
    minor_units: 1_000_000_000_000_000_000,
    name: "Uniswap",
    symbol: "UNI",
    symbol_first: false,
};
pub const USDC: &'static Currency = &Currency {
    exponent: 6,
    code: "USDC",
    locale: EnUs,
    minor_units: 1_000_000,
    name: "USD Coin",
    symbol: "USDC",
    symbol_first: false,
};
pub const USDT: &'static Currency = &Currency {
    exponent: 6,
    code: "USDT",
    locale: EnUs,
    minor_units: 1_000_000,
    name: "Tether",
    symbol: "USDT",
    symbol_first: false,
};
pub const XTZ: &'static Currency = &Currency {
    exponent: 6,
    code: "XTZ",
    locale: EnUs,
    minor_units: 1_000_000,
    name: "Tezos",
    symbol: "XTZ",
    symbol_first: false,
};
pub const ZEC: &'static Currency = &Currency {
    exponent: 8,
    code: "ZEC",
    locale: EnUs,
    minor_units: 100_000_000,
    name: "ZCash",
    symbol: "ZEC",
    symbol_first: false,
};
pub const BCH: &'static Currency = &Currency {
    exponent: 8,
    code: "BCH",
    locale: EnUs,
    minor_units: 100_000_000,
    name: "Bitcoin Cash",
    symbol: "BCH",
    symbol_first: false,
};
pub const BSV: &'static Currency = &Currency {
    exponent: 8,
    code: "BSV",
    locale: EnUs,
    minor_units: 100_000_000,
    name: "Bitcoin SV",
    symbol: "BSV",
    symbol_first: false,
};
