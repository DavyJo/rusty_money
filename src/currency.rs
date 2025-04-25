use crate::currencies::iso::{AED, AFN, ALL, AMD, ANG, AOA, ARS, AUD, AWG, AZN, BAM, BBD, BDT, BGN, BHD, BIF, BMD, BND, BOB, BRL, BSD, BTN, BWP, BYN, BYR, BZD, CAD, CDF, CHF, CLF, CLP, CNY, COP, CRC, CUC, CUP, CVE, CZK, DJF, DKK, DOP, DZD, EGP, ERN, ETB, EUR, FJD, FKP, GBP, GEL, GHS, GIP, GMD, GNF, GTQ, GYD, HKD, HNL, HRK, HTG, HUF, IDR, ILS, INR, IQD, IRR, ISK, JMD, JOD, JPY, KES, KGS, KHR, KMF, KPW, KRW, KWD, KYD, KZT, LAK, LBP, LKR, LRD, LSL, LYD, MAD, MDL, MGA, MKD, MMK, MNT, MOP, MRU, MUR, MVR, MWK, MXN, MYR, MZN, NAD, NGN, NIO, NOK, NPR, NZD, OMR, PAB, PEN, PGK, PHP, PKR, PLN, PYG, QAR, ROL, RON, RSD, RUB, RWF, SAR, SBD, SCR, SDG, SEK, SGD, SHP, SKK, SLE, SLL, SOS, SRD, SSP, STD, STN, SVC, SYP, SZL, THB, TJS, TMT, TND, TOP, TRY, TTD, TWD, TZS, UAH, UGX, USD, UYU, UYW, UZS, VES, VND, VUV, WST, XAF, XAG, XAU, XBA, XBB, XBC, XBD, XCD, XDR, XOF, XPD, XPF, XPT, XTS, YER, ZAR, ZMK, ZMW, ZWL};
use crate::{Locale, Money};
use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use serde::{Deserialize, Serialize};

/// Pre-requisite for a Currency to be accepted by a Money.
pub trait FormattableCurrency: PartialEq + Eq + Copy {
    fn exponent(&self) -> u32;

    fn code(&self) -> &'static str;

    fn locale(&self) -> Locale;

    fn symbol(&self) -> &'static str;

    fn symbol_first(&self) -> bool;
}

#[derive(Debug, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Currency {
    pub code: &'static str,
    pub exponent: u32,
    pub locale: Locale,
    pub minor_units: u64,
    pub name: &'static str,
    pub symbol: &'static str,
    pub symbol_first: bool,
}
impl PartialEq<Self> for Currency {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl PartialOrd<Self> for Currency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.code.partial_cmp(other.code)
    }
}

impl Ord for Currency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.code.cmp(other.code)
    }
}

impl Sum for Money {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        if let Some(first) = iter.next() {
            iter.fold(first, |acc, money| acc + money)
        } else {
            Money::from_major(0, EUR)
        }
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency {
            code: "",
            exponent: 0,
            locale: Locale::EnEu,
            minor_units: 0,
            name: "",
            symbol: "",
            symbol_first: false,
        }
    }
}

impl FormattableCurrency for Currency {
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

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

/// Lookup über match (perfekte Kompilierzeit-Optimierung)
pub fn find_currency(code: &str) -> Option<Currency> {
    match code {
        "AED" => Some(AED),
        "AFN" => Some(AFN),
        "ALL" => Some(ALL),
        "AMD" => Some(AMD),
        "ANG" => Some(ANG),
        "AOA" => Some(AOA),
        "ARS" => Some(ARS),
        "AUD" => Some(AUD),
        "AWG" => Some(AWG),
        "AZN" => Some(AZN),
        "BAM" => Some(BAM),
        "BBD" => Some(BBD),
        "BDT" => Some(BDT),
        "BGN" => Some(BGN),
        "BHD" => Some(BHD),
        "BIF" => Some(BIF),
        "BMD" => Some(BMD),
        "BND" => Some(BND),
        "BOB" => Some(BOB),
        "BRL" => Some(BRL),
        "BSD" => Some(BSD),
        "BTN" => Some(BTN),
        "BWP" => Some(BWP),
        "BYN" => Some(BYN),
        "BYR" => Some(BYR),
        "BZD" => Some(BZD),
        "CAD" => Some(CAD),
        "CDF" => Some(CDF),
        "CHF" => Some(CHF),
        "CLF" => Some(CLF),
        "CLP" => Some(CLP),
        "CNY" => Some(CNY),
        "COP" => Some(COP),
        "CRC" => Some(CRC),
        "CUC" => Some(CUC),
        "CUP" => Some(CUP),
        "CVE" => Some(CVE),
        "CZK" => Some(CZK),
        "DJF" => Some(DJF),
        "DKK" => Some(DKK),
        "DOP" => Some(DOP),
        "DZD" => Some(DZD),
        "EGP" => Some(EGP),
        "ERN" => Some(ERN),
        "ETB" => Some(ETB),
        "EUR" => Some(EUR),
        "FJD" => Some(FJD),
        "FKP" => Some(FKP),
        "GBP" => Some(GBP),
        "GEL" => Some(GEL),
        "GHS" => Some(GHS),
        "GIP" => Some(GIP),
        "GMD" => Some(GMD),
        "GNF" => Some(GNF),
        "GTQ" => Some(GTQ),
        "GYD" => Some(GYD),
        "HKD" => Some(HKD),
        "HNL" => Some(HNL),
        "HRK" => Some(HRK),
        "HTG" => Some(HTG),
        "HUF" => Some(HUF),
        "IDR" => Some(IDR),
        "ILS" => Some(ILS),
        "INR" => Some(INR),
        "IQD" => Some(IQD),
        "IRR" => Some(IRR),
        "ISK" => Some(ISK),
        "JMD" => Some(JMD),
        "JOD" => Some(JOD),
        "JPY" => Some(JPY),
        "KES" => Some(KES),
        "KGS" => Some(KGS),
        "KHR" => Some(KHR),
        "KMF" => Some(KMF),
        "KPW" => Some(KPW),
        "KRW" => Some(KRW),
        "KWD" => Some(KWD),
        "KYD" => Some(KYD),
        "KZT" => Some(KZT),
        "LAK" => Some(LAK),
        "LBP" => Some(LBP),
        "LKR" => Some(LKR),
        "LRD" => Some(LRD),
        "LSL" => Some(LSL),
        "LYD" => Some(LYD),
        "MAD" => Some(MAD),
        "MDL" => Some(MDL),
        "MGA" => Some(MGA),
        "MKD" => Some(MKD),
        "MMK" => Some(MMK),
        "MNT" => Some(MNT),
        "MOP" => Some(MOP),
        "MRU" => Some(MRU),
        "MUR" => Some(MUR),
        "MVR" => Some(MVR),
        "MWK" => Some(MWK),
        "MXN" => Some(MXN),
        "MYR" => Some(MYR),
        "MZN" => Some(MZN),
        "NAD" => Some(NAD),
        "NGN" => Some(NGN),
        "NIO" => Some(NIO),
        "NOK" => Some(NOK),
        "NPR" => Some(NPR),
        "NZD" => Some(NZD),
        "OMR" => Some(OMR),
        "PAB" => Some(PAB),
        "PEN" => Some(PEN),
        "PGK" => Some(PGK),
        "PHP" => Some(PHP),
        "PKR" => Some(PKR),
        "PLN" => Some(PLN),
        "PYG" => Some(PYG),
        "QAR" => Some(QAR),
        "RON" => Some(RON),
        "ROL" => Some(ROL),
        "RSD" => Some(RSD),
        "RUB" => Some(RUB),
        "RWF" => Some(RWF),
        "SAR" => Some(SAR),
        "SBD" => Some(SBD),
        "SCR" => Some(SCR),
        "SDG" => Some(SDG),
        "SEK" => Some(SEK),
        "SGD" => Some(SGD),
        "SHP" => Some(SHP),
        "SKK" => Some(SKK),
        "SLE" => Some(SLE),
        "SLL" => Some(SLL),
        "SOS" => Some(SOS),
        "SRD" => Some(SRD),
        "SSP" => Some(SSP),
        "STD" => Some(STD),
        "STN" => Some(STN),
        "SVC" => Some(SVC),
        "SYP" => Some(SYP),
        "SZL" => Some(SZL),
        "THB" => Some(THB),
        "TJS" => Some(TJS),
        "TMT" => Some(TMT),
        "TND" => Some(TND),
        "TOP" => Some(TOP),
        "TRY" => Some(TRY),
        "TTD" => Some(TTD),
        "TWD" => Some(TWD),
        "TZS" => Some(TZS),
        "UAH" => Some(UAH),
        "UGX" => Some(UGX),
        "USD" => Some(USD),
        "UYU" => Some(UYU),
        "UYW" => Some(UYW),
        "UZS" => Some(UZS),
        "VES" => Some(VES),
        "VND" => Some(VND),
        "VUV" => Some(VUV),
        "WST" => Some(WST),
        "XAF" => Some(XAF),
        "XAG" => Some(XAG),
        "XAU" => Some(XAU),
        "XBA" => Some(XBA),
        "XBB" => Some(XBB),
        "XBC" => Some(XBC),
        "XBD" => Some(XBD),
        "XCD" => Some(XCD),
        "XDR" => Some(XDR),
        "XOF" => Some(XOF),
        "XPD" => Some(XPD),
        "XPF" => Some(XPF),
        "XPT" => Some(XPT),
        "XTS" => Some(XTS),
        "YER" => Some(YER),
        "ZAR" => Some(ZAR),
        "ZMK" => Some(ZMK),
        "ZMW" => Some(ZMW),
        "ZWL" => Some(ZWL),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::currencies::crypto::BTC;
    use crate::currencies::iso::{AED, EUR, USD};
    use crate::{find_currency, Money};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn currencies_in_different_modules_are_not_equal() {
        assert_eq!(USD.code, "USD");
        assert_eq!(AED.code, "AED");
    }

    #[test]
    fn find_works_in_modules() {
        assert_eq!(find_currency("USD").unwrap().code, "USD");
        assert_eq!(find_currency("AED").unwrap().code, "AED");
    }

    #[test]
    fn find_returns_known_currencies() {
        let currency_by_alpha = USD;
        assert_eq!(currency_by_alpha.code, "USD");
        assert_eq!(currency_by_alpha.exponent, 2);
        assert_eq!(currency_by_alpha.symbol, "$");
    }

    #[test]
    fn currency_can_be_accessed_by_reference() {
        assert_eq!(USD.code, "USD");
        assert_eq!(USD.exponent, 2);
        assert_eq!(USD.symbol, "$");
    }

    #[test]
    fn test_sum() {
        let money1 = Money::from_decimal(Decimal::from_str("10.1").unwrap(), USD);
        let money2 = Money::from_decimal(Decimal::from_str("20.2").unwrap(), USD);
        let money3 = Money::from_decimal(Decimal::from_str("30.3").unwrap(), USD);

        let monies = vec![money1, money2, money3];

        let total: Money = monies.into_iter().sum();

        assert_eq!(total.amount(), &Decimal::from_str("60.6").unwrap());
        assert_eq!(total.currency(), &USD);
    }

    #[test]
    #[should_panic]
    fn test_fail_sum() {
        let money1 = Money::from_decimal(Decimal::from_str("10.1").unwrap(), USD);
        let money2 = Money::from_decimal(Decimal::from_str("20.2").unwrap(), USD);
        let money3 = Money::from_decimal(Decimal::from_str("30.3").unwrap(), EUR);

        let monies = vec![money1, money2, money3];

        let _total: Money = monies.into_iter().sum();
    }

    #[test]
    fn find_returns_known_currencies_crypto() {
        let currency_by_code = BTC;
        assert_eq!(currency_by_code.code, "BTC");
        assert_eq!(currency_by_code.exponent, 8);
        assert_eq!(currency_by_code.symbol, "₿");
    }
}
