/// Crypto Currency Set
pub mod crypto {
    use crate::Currency;

    pub const BTC: &'static Currency = &Currency {
        code: "BTC",
        exponent: 8,
        locale: crate::Locale::EnUs,
        minor_units: 100_000_000,
        name: "Bitcoin",
        symbol: "₿",
        symbol_first: true,
    };
    pub const COMP: &'static Currency = &Currency {
        code: "COMP",
        exponent: 18,
        locale: crate::Locale::EnUs,
        minor_units: 1_000_000_000_000_000_000,
        name: "Compound",
        symbol: "COMP",
        symbol_first: false,
    };
    pub const DAI: &'static Currency = &Currency {
        code: "DAI",
        exponent: 18,
        locale: crate::Locale::EnUs,
        minor_units: 1_000_000_000_000_000_000,
        name: "Dai Stablecoin",
        symbol: "DAI",
        symbol_first: false,
    };
    pub const ETH: &'static Currency = &Currency {
        code: "ETH",
        exponent: 18,
        locale: crate::Locale::EnUs,
        minor_units: 1_000_000_000_000_000_000,
        name: "Ethereum",
        symbol: "ETH",
        symbol_first: false,
    };
    pub const MKR: &'static Currency = &Currency {
        code: "MKR",
        exponent: 18,
        locale: crate::Locale::EnUs,
        minor_units: 1_000_000_000_000_000_000,
        name: "Maker",
        symbol: "MKR",
        symbol_first: false,
    };
    pub const UNI: &'static Currency = &Currency {
        code: "UNI",
        exponent: 18,
        locale: crate::Locale::EnUs,
        minor_units: 1_000_000_000_000_000_000,
        name: "Uniswap",
        symbol: "UNI",
        symbol_first: false,
    };
    pub const USDC: &'static Currency = &Currency {
        code: "USDC",
        exponent: 6,
        locale: crate::Locale::EnUs,
        minor_units: 1_000_000,
        name: "USD Coin",
        symbol: "USDC",
        symbol_first: false,
    };
    pub const USDT: &'static Currency = &Currency {
        code: "USDT",
        exponent: 6,
        locale: crate::Locale::EnUs,
        minor_units: 1_000_000,
        name: "Tether",
        symbol: "USDT",
        symbol_first: false,
    };
    pub const XTZ: &'static Currency = &Currency {
        code: "XTZ",
        exponent: 6,
        locale: crate::Locale::EnUs,
        minor_units: 1_000_000,
        name: "Tezos",
        symbol: "XTZ",
        symbol_first: false,
    };
    pub const ZEC: &'static Currency = &Currency {
        code: "ZEC",
        exponent: 8,
        locale: crate::Locale::EnUs,
        minor_units: 100_000_000,
        name: "ZCash",
        symbol: "ZEC",
        symbol_first: false,
    };
    // https://www.bitcoincash.org/
    pub const BCH: &'static Currency = &Currency {
        code: "BCH",
        exponent: 8,
        locale: crate::Locale::EnUs,
        minor_units: 100_000_000,
        name: "Bitcoin Cash",
        symbol: "BCH",
        symbol_first: false,
    };
    // https://bitcoinsv.com/
    pub const BSV: &'static Currency = &Currency {
        code: "BSV",
        exponent: 8,
        locale: crate::Locale::EnUs,
        minor_units: 100_000_000,
        name: "Bitcoin SV",
        symbol: "BSV",
        symbol_first: false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find_currency;

    #[test]
    fn find_returns_known_currencies() {
        let currency_by_code = find_currency("BTC").unwrap();
        assert_eq!(currency_by_code.code, "BTC");
        assert_eq!(currency_by_code.exponent, 8);
        assert_eq!(currency_by_code.symbol, "₿");
    }

    #[test]
    fn find_returns_none_on_unknown_currency() {
        assert_eq!(find_currency("fake"), None);
    }

    #[test]
    fn currency_can_be_accessed_by_reference() {
        assert_eq!(crypto::ETH.code, "ETH");
        assert_eq!(crypto::ETH.exponent, 18);
        assert_eq!(crypto::ETH.symbol, "ETH");
    }

    #[test]
    fn find_and_reference_point_to_same() {
        assert_eq!(crypto::BTC, find_currency("BTC").unwrap());
    }
}
