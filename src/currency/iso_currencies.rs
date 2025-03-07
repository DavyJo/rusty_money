pub mod iso {
    use crate::Currency;
    use crate::Locale::{EnBy, EnEu, EnIn, EnUs};

    pub const AED: &'static Currency = &Currency {
        code: "AED",
        exponent: 2,
        locale: EnUs,
        minor_units: 25,
        name: "United Arab Emirates Dirham",
        symbol: "د.إ",
        symbol_first: false,
    };
    pub const AFN: &'static Currency = &Currency {
        code: "AFN",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Afghan Afghani",
        symbol: "؋",
        symbol_first: false,
    };
    pub const ALL: &'static Currency = &Currency {
        code: "ALL",
        exponent: 2,
        locale: EnEu,
        minor_units: 1,
        name: "Albanian lek",
        symbol: "L",
        symbol_first: false,
    };
    pub const AMD: &'static Currency = &Currency {
        code: "AMD",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "Armenian Dram",
        symbol: "դր.",
        symbol_first: false,
    };
    pub const ANG: &'static Currency = &Currency {
        code: "ANG",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Netherlands Antillean Gulden",
        symbol: "ƒ",
        symbol_first: false,
    };
    pub const AOA: &'static Currency = &Currency {
        code: "AOA",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "Angolan Kwanza",
        symbol: "Kz",
        symbol_first: false,
    };
    pub const ARS: &'static Currency = &Currency {
        code: "ARS",
        exponent: 2,
        locale: EnEu,
        minor_units: 1,
        name: "Argentine Peso",
        symbol: "$",
        symbol_first: true,
    };
    pub const AUD: &'static Currency = &Currency {
        code: "AUD",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Australian Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const AWG: &'static Currency = &Currency {
        code: "AWG",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Aruban Florin",
        symbol: "ƒ",
        symbol_first: false,
    };
    pub const AZN: &'static Currency = &Currency {
        code: "AZN",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Azerbaijani Manat",
        symbol: "₼",
        symbol_first: true,
    };
    pub const BAM: &'static Currency = &Currency {
        code: "BAM",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Bosnia and Herzegovina Convertible Mark",
        symbol: "KM",
        symbol_first: true,
    };
    pub const BBD: &'static Currency = &Currency {
        code: "BBD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Barbadian Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const BDT: &'static Currency = &Currency {
        code: "BDT",
        exponent: 2,
        locale: EnIn,
        minor_units: 1,
        name: "Bangladeshi Taka",
        symbol: "৳",
        symbol_first: true,
    };
    pub const BGN: &'static Currency = &Currency {
        code: "BGN",
        exponent: 2,
        locale: EnIn,
        minor_units: 1,
        name: "Bulgarian Lev",
        symbol: "лв.",
        symbol_first: false,
    };
    pub const BHD: &'static Currency = &Currency {
        code: "BHD",
        exponent: 3,
        locale: EnUs,
        minor_units: 5,
        name: "Bahraini Dinar",
        symbol: "د.ب",
        symbol_first: true,
    };
    pub const BIF: &'static Currency = &Currency {
        code: "BIF",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Burundian Franc",
        symbol: "Fr",
        symbol_first: false,
    };
    pub const BMD: &'static Currency = &Currency {
        code: "BMD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Bermudian Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const BND: &'static Currency = &Currency {
        code: "BND",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Brunei Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const BOB: &'static Currency = &Currency {
        code: "BOB",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "Bolivian Boliviano",
        symbol: "Bs.",
        symbol_first: true,
    };
    pub const BRL: &'static Currency = &Currency {
        code: "BRL",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Brazilian real",
        symbol: "R$",
        symbol_first: true,
    };
    pub const BSD: &'static Currency = &Currency {
        code: "BSD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Bahamian Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const BTN: &'static Currency = &Currency {
        code: "BTN",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Bhutanese Ngultrum",
        symbol: "Nu.",
        symbol_first: false,
    };
    pub const BWP: &'static Currency = &Currency {
        code: "BWP",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Botswana Pula",
        symbol: "P",
        symbol_first: true,
    };
    pub const BYN: &'static Currency = &Currency {
        code: "BYN",
        exponent: 2,
        locale: EnBy,
        minor_units: 1,
        name: "Belarusian Ruble",
        symbol: "Br",
        symbol_first: false,
    };
    pub const BYR: &'static Currency = &Currency {
        code: "BYR",
        exponent: 0,
        locale: EnBy,
        minor_units: 100,
        name: "Belarusian Ruble",
        symbol: "Br",
        symbol_first: false,
    };
    pub const BZD: &'static Currency = &Currency {
        code: "BZD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Belize Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const CAD: &'static Currency = &Currency {
        code: "CAD",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Canadian Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const CDF: &'static Currency = &Currency {
        code: "CDF",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Congolese Franc",
        symbol: "Fr",
        symbol_first: false,
    };
    pub const CHF: &'static Currency = &Currency {
        code: "CHF",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Swiss Franc",
        symbol: "Fr",
        symbol_first: true,
    };
    pub const CLF: &'static Currency = &Currency {
        code: "CLF",
        exponent: 4,
        locale: EnEu,
        minor_units: 5,
        name: "Unidad de Fomento",
        symbol: "UF",
        symbol_first: true,
    };
    pub const CLP: &'static Currency = &Currency {
        code: "CLP",
        exponent: 0,
        locale: EnEu,
        minor_units: 1,
        name: "Chilean Peso",
        symbol: "$",
        symbol_first: true,
    };
    pub const CNY: &'static Currency = &Currency {
        code: "CNY",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Chinese Renminbi Yuan",
        symbol: "¥",
        symbol_first: true,
    };
    pub const COP: &'static Currency = &Currency {
        code: "COP",
        exponent: 2,
        locale: EnEu,
        minor_units: 20,
        name: "Colombian Peso",
        symbol: "$",
        symbol_first: true,
    };
    pub const CRC: &'static Currency = &Currency {
        code: "CRC",
        exponent: 2,
        locale: EnEu,
        minor_units: 500, // TODO - Investigate
        name: "Costa Rican Colón",
        symbol: "₡",
        symbol_first: true,
    };
    pub const CUC: &'static Currency = &Currency {
        code: "CUC",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Cuban Convertible Peso",
        symbol: "$",
        symbol_first: false,
    };
    pub const CUP: &'static Currency = &Currency {
        code: "CUP",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Cuban Peso",
        symbol: "$",
        symbol_first: true,
    };
    pub const CVE: &'static Currency = &Currency {
        code: "CVE",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Cape Verdean Escudo",
        symbol: "$",
        symbol_first: false,
    };
    pub const CZK: &'static Currency = &Currency {
        code: "CZK",
        exponent: 2,
        locale: EnBy,
        minor_units: 100,
        name: "Czech Koruna",
        symbol: "Kč",
        symbol_first: false,
    };
    pub const DJF: &'static Currency = &Currency {
        code: "DJF",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Djiboutian Franc",
        symbol: "Fdj",
        symbol_first: false,
    };
    pub const DKK: &'static Currency = &Currency {
        code: "DKK",
        exponent: 2,
        locale: EnEu,
        minor_units: 50,
        name: "Danish Krone",
        symbol: "kr.",
        symbol_first: false,
    };
    pub const DOP: &'static Currency = &Currency {
        code: "DOP",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Dominican Peso",
        symbol: "$",
        symbol_first: true,
    };
    pub const DZD: &'static Currency = &Currency {
        code: "DZD",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Algerian Dinar",
        symbol: "د.ج",
        symbol_first: false,
    };
    pub const EGP: &'static Currency = &Currency {
        code: "EGP",
        exponent: 2,
        locale: EnUs,
        minor_units: 25,
        name: "Egyptian Pound",
        symbol: "ج.م",
        symbol_first: true,
    };
    pub const ERN: &'static Currency = &Currency {
        code: "ERN",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Eritrean Nakfa",
        symbol: "Nfk",
        symbol_first: false,
    };
    pub const ETB: &'static Currency = &Currency {
        code: "ETB",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Ethiopian Birr",
        symbol: "Br",
        symbol_first: false,
    };
    pub const EUR: &'static Currency = &Currency {
        code: "EUR",
        exponent: 2,
        locale: EnEu,
        minor_units: 1,
        name: "Euro",
        symbol: "€",
        symbol_first: true,
    };
    pub const FJD: &'static Currency = &Currency {
        code: "FJD",
        exponent: 2,
        locale: EnEu,
        minor_units: 5,
        name: "Fijian Dollar",
        symbol: "$",
        symbol_first: false,
    };
    pub const FKP: &'static Currency = &Currency {
        code: "FKP",
        exponent: 2,
        locale: EnEu,
        minor_units: 1,
        name: "Falkland Pound",
        symbol: "£",
        symbol_first: false,
    };
    pub const GBP: &'static Currency = &Currency {
        code: "GBP",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "British Pound",
        symbol: "£",
        symbol_first: true,
    };
    pub const GEL: &'static Currency = &Currency {
        code: "GEL",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Georgian Lari",
        symbol: "ლ",
        symbol_first: false,
    };
    pub const GHS: &'static Currency = &Currency {
        code: "GHS",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Ghanaian Cedi",
        symbol: "₵",
        symbol_first: true,
    };
    pub const GIP: &'static Currency = &Currency {
        code: "GIP",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Gibraltar Pound",
        symbol: "£",
        symbol_first: true,
    };
    pub const GMD: &'static Currency = &Currency {
        code: "GMD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Gambian Dalasi",
        symbol: "D",
        symbol_first: false,
    };
    pub const GNF: &'static Currency = &Currency {
        code: "GNF",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Guinean Franc",
        symbol: "Fr",
        symbol_first: false,
    };
    pub const GTQ: &'static Currency = &Currency {
        code: "GTQ",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Guatemalan Quetzal",
        symbol: "Q",
        symbol_first: true,
    };
    pub const GYD: &'static Currency = &Currency {
        code: "GYD",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Guyanese Dollar",
        symbol: "$",
        symbol_first: false,
    };
    pub const HKD: &'static Currency = &Currency {
        code: "HKD",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "Hong Kong Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const HNL: &'static Currency = &Currency {
        code: "HNL",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Honduran Lempira",
        symbol: "L",
        symbol_first: true,
    };
    pub const HRK: &'static Currency = &Currency {
        code: "HRK",
        exponent: 2,
        locale: EnEu,
        minor_units: 1,
        name: "Croatian Kuna",
        symbol: "kn",
        symbol_first: false,
    };
    pub const HTG: &'static Currency = &Currency {
        code: "HTG",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Haitian Gourde",
        symbol: "G",
        symbol_first: false,
    };
    pub const HUF: &'static Currency = &Currency {
        code: "HUF",
        exponent: 0,
        locale: EnBy,
        minor_units: 5,
        name: "Hungarian Forint",
        symbol: "Ft",
        symbol_first: false,
    };
    pub const IDR: &'static Currency = &Currency {
        code: "IDR",
        exponent: 2,
        locale: EnUs,
        minor_units: 5000,
        name: "Indonesian Rupiah",
        symbol: "Rp",
        symbol_first: true,
    };
    pub const ILS: &'static Currency = &Currency {
        code: "ILS",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "Israeli New Sheqel",
        symbol: "₪",
        symbol_first: true,
    };
    pub const INR: &'static Currency = &Currency {
        code: "INR",
        exponent: 2,
        locale: EnIn,
        minor_units: 50,
        name: "Indian Rupee",
        symbol: "₹",
        symbol_first: true,
    };
    pub const IQD: &'static Currency = &Currency {
        code: "IQD",
        exponent: 3,
        locale: EnUs,
        minor_units: 50000,
        name: "Iraqi Dinar",
        symbol: "ع.د",
        symbol_first: false,
    };
    pub const IRR: &'static Currency = &Currency {
        code: "IRR",
        exponent: 2,
        locale: EnUs,
        minor_units: 5000,
        name: "Iranian Rial",
        symbol: "﷼",
        symbol_first: true,
    };
    pub const ISK: &'static Currency = &Currency {
        code: "ISK",
        exponent: 0,
        locale: EnEu,
        minor_units: 1,
        name: "Icelandic Króna",
        symbol: "kr.",
        symbol_first: true,
    };
    pub const JMD: &'static Currency = &Currency {
        code: "JMD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Jamaican Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const JOD: &'static Currency = &Currency {
        code: "JOD",
        exponent: 3,
        locale: EnUs,
        minor_units: 5,
        name: "Jordanian Dinar",
        symbol: "د.ا",
        symbol_first: true,
    };
    pub const JPY: &'static Currency = &Currency {
        code: "JPY",
        exponent: 0,
        locale: EnUs,
        minor_units: 1,
        name: "Japanese Yen",
        symbol: "¥",
        symbol_first: true,
    };
    pub const KES: &'static Currency = &Currency {
        code: "KES",
        exponent: 2,
        locale: EnUs,
        minor_units: 50,
        name: "Kenyan Shilling",
        symbol: "KSh",
        symbol_first: true,
    };
    pub const KGS: &'static Currency = &Currency {
        code: "KGS",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Kyrgyzstani Som",
        symbol: "som",
        symbol_first: false,
    };
    pub const KHR: &'static Currency = &Currency {
        code: "KHR",
        exponent: 2,
        locale: EnUs,
        minor_units: 5000,
        name: "Cambodian Riel",
        symbol: "៛",
        symbol_first: false,
    };
    pub const KMF: &'static Currency = &Currency {
        code: "KMF",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Comorian Franc",
        symbol: "Fr",
        symbol_first: false,
    };
    pub const KPW: &'static Currency = &Currency {
        code: "KPW",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "North Korean Won",
        symbol: "₩",
        symbol_first: false,
    };
    pub const KRW: &'static Currency = &Currency {
        code: "KRW",
        exponent: 0,
        locale: EnUs,
        minor_units: 1,
        name: "South Korean Won",
        symbol: "₩",
        symbol_first: true,
    };
    pub const KWD: &'static Currency = &Currency {
        code: "KWD",
        exponent: 3,
        locale: EnUs,
        minor_units: 5,
        name: "Kuwaiti Dinar",
        symbol: "د.ك",
        symbol_first: true,
    };
    pub const KYD: &'static Currency = &Currency {
        code: "KYD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Cayman Islands Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const KZT: &'static Currency = &Currency {
        code: "KZT",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Kazakhstani Tenge",
        symbol: "₸",
        symbol_first: false,
    };
    pub const LAK: &'static Currency = &Currency {
        code: "LAK",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "Lao Kip",
        symbol: "₭",
        symbol_first: false,
    };
    pub const LBP: &'static Currency = &Currency {
        code: "LBP",
        exponent: 2,
        locale: EnUs,
        minor_units: 25000,
        name: "Lebanese Pound",
        symbol: "ل.ل",
        symbol_first: true,
    };
    pub const LKR: &'static Currency = &Currency {
        code: "LKR",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Sri Lankan Rupee",
        symbol: "₨",
        symbol_first: false,
    };
    pub const LRD: &'static Currency = &Currency {
        code: "LRD",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Liberian Dollar",
        symbol: "$",
        symbol_first: false,
    };
    pub const LSL: &'static Currency = &Currency {
        code: "LSL",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Lesotho Loti",
        symbol: "L",
        symbol_first: false,
    };
    pub const LYD: &'static Currency = &Currency {
        code: "LYD",
        exponent: 3,
        locale: EnUs,
        minor_units: 50,
        name: "Libyan Dinar",
        symbol: "ل.د",
        symbol_first: false,
    };
    pub const MAD: &'static Currency = &Currency {
        code: "MAD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Moroccan Dirham",
        symbol: "د.م.",
        symbol_first: false,
    };
    pub const MDL: &'static Currency = &Currency {
        code: "MDL",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Moldovan Leu",
        symbol: "L",
        symbol_first: false,
    };
    pub const MGA: &'static Currency = &Currency {
        code: "MGA",
        exponent: 1, // TODO - exponent is 1/5th need to represent somehow
        locale: EnUs,
        minor_units: 1,
        name: "Malagasy Ariary",
        symbol: "Ar",
        symbol_first: true,
    };
    pub const MKD: &'static Currency = &Currency {
        code: "MKD",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Macedonian Denar",
        symbol: "ден",
        symbol_first: false,
    };
    pub const MMK: &'static Currency = &Currency {
        code: "MMK",
        exponent: 2,
        locale: EnUs,
        minor_units: 50,
        name: "Myanmar Kyat",
        symbol: "K",
        symbol_first: false,
    };
    pub const MNT: &'static Currency = &Currency {
        code: "MNT",
        exponent: 2,
        locale: EnUs,
        minor_units: 2000,
        name: "Mongolian Tögrög",
        symbol: "₮",
        symbol_first: false,
    };
    pub const MOP: &'static Currency = &Currency {
        code: "MOP",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "Macanese Pataca",
        symbol: "P",
        symbol_first: false,
    };
    pub const MRU: &'static Currency = &Currency {
        code: "MRU",
        exponent: 1, // TODO - exponent problem of 5
        locale: EnUs,
        minor_units: 1,
        name: "Mauritanian Ouguiya",
        symbol: "UM",
        symbol_first: false,
    };
    pub const MUR: &'static Currency = &Currency {
        code: "MUR",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Mauritian Rupee",
        symbol: "₨",
        symbol_first: true,
    };
    pub const MVR: &'static Currency = &Currency {
        code: "MVR",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Maldivian Rufiyaa",
        symbol: "MVR",
        symbol_first: false,
    };
    pub const MWK: &'static Currency = &Currency {
        code: "MWK",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Malawian Kwacha",
        symbol: "MK",
        symbol_first: false,
    };
    pub const MXN: &'static Currency = &Currency {
        code: "MXN",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Mexican Peso",
        symbol: "$",
        symbol_first: true,
    };
    pub const MYR: &'static Currency = &Currency {
        code: "MYR",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Malaysian Ringgit",
        symbol: "RM",
        symbol_first: true,
    };
    pub const MZN: &'static Currency = &Currency {
        code: "MZN",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Mozambican Metical",
        symbol: "MTn",
        symbol_first: true,
    };
    pub const NAD: &'static Currency = &Currency {
        code: "NAD",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Namibian Dollar",
        symbol: "$",
        symbol_first: false,
    };
    pub const NGN: &'static Currency = &Currency {
        code: "NGN",
        exponent: 2,
        locale: EnUs,
        minor_units: 50,
        name: "Nigerian Naira",
        symbol: "₦",
        symbol_first: true,
    };
    pub const NIO: &'static Currency = &Currency {
        code: "NIO",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Nicaraguan Córdoba",
        symbol: "C$",
        symbol_first: true,
    };
    pub const NOK: &'static Currency = &Currency {
        code: "NOK",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Norwegian Krone",
        symbol: "kr",
        symbol_first: false,
    };
    pub const NPR: &'static Currency = &Currency {
        code: "NPR",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Nepalese Rupee",
        symbol: "रु",
        symbol_first: true,
    };
    pub const NZD: &'static Currency = &Currency {
        code: "NZD",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "New Zealand Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const OMR: &'static Currency = &Currency {
        code: "OMR",
        exponent: 3,
        locale: EnUs,
        minor_units: 5,
        name: "Omani Rial",
        symbol: "ر.ع.",
        symbol_first: true,
    };
    pub const PAB: &'static Currency = &Currency {
        code: "PAB",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Panamanian Balboa",
        symbol: "B/.",
        symbol_first: true,
    };
    pub const PEN: &'static Currency = &Currency {
        code: "PEN",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Peruvian Sol",
        symbol: "S/",
        symbol_first: true,
    };
    pub const PGK: &'static Currency = &Currency {
        code: "PGK",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Papua New Guinean Kina",
        symbol: "K",
        symbol_first: false,
    };
    pub const PHP: &'static Currency = &Currency {
        code: "PHP",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Philippine Peso",
        symbol: "₱",
        symbol_first: true,
    };
    pub const PKR: &'static Currency = &Currency {
        code: "PKR",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Pakistani Rupee",
        symbol: "₨",
        symbol_first: true,
    };
    pub const PLN: &'static Currency = &Currency {
        code: "PLN",
        exponent: 2,
        locale: EnBy,
        minor_units: 1,
        name: "Polish Złoty",
        symbol: "zł",
        symbol_first: false,
    };
    pub const PYG: &'static Currency = &Currency {
        code: "PYG",
        exponent: 0,
        locale: EnBy,
        minor_units: 5000,
        name: "Paraguayan Guaraní",
        symbol: "₲",
        symbol_first: true,
    };
    pub const QAR: &'static Currency = &Currency {
        code: "QAR",
        exponent: 2,
        locale: EnBy,
        minor_units: 1,
        name: "Qatari Riyal",
        symbol: "ر.ق",
        symbol_first: false,
    };
    pub const RON: &'static Currency = &Currency {
        code: "RON",
        exponent: 2,
        locale: EnEu,
        minor_units: 1,
        name: "Romanian Leu",
        symbol: "RON",
        symbol_first: false,
    };
    pub const ROL: &'static Currency = &Currency {
        code: "ROL",
        exponent: 0,
        locale: EnEu,
        minor_units: 1,
        name: "Romanian Leu",
        symbol: "ROL",
        symbol_first: false,
    };
    pub const RSD: &'static Currency = &Currency {
        code: "RSD",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Serbian Dinar",
        symbol: "РСД",
        symbol_first: true,
    };
    pub const RUB: &'static Currency = &Currency {
        code: "RUB",
        exponent: 2,
        locale: EnEu,
        minor_units: 1,
        name: "Russian Ruble",
        symbol: "₽",
        symbol_first: false,
    };
    pub const RWF: &'static Currency = &Currency {
        code: "RWF",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Rwandan Franc",
        symbol: "FRw",
        symbol_first: false,
    };
    pub const SAR: &'static Currency = &Currency {
        code: "SAR",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Saudi Riyal",
        symbol: "ر.س",
        symbol_first: true,
    };
    pub const SBD: &'static Currency = &Currency {
        code: "SBD",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "Solomon Islands Dollar",
        symbol: "$",
        symbol_first: false,
    };
    pub const SCR: &'static Currency = &Currency {
        code: "SCR",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Seychellois Rupee",
        symbol: "₨",
        symbol_first: false,
    };
    pub const SDG: &'static Currency = &Currency {
        code: "SDG",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Sudanese Pound",
        symbol: "£",
        symbol_first: true,
    };
    pub const SEK: &'static Currency = &Currency {
        code: "SEK",
        exponent: 2,
        locale: EnBy,
        minor_units: 100,
        name: "Swedish Krona",
        symbol: "kr",
        symbol_first: false,
    };
    pub const SGD: &'static Currency = &Currency {
        code: "SGD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Singapore Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const SHP: &'static Currency = &Currency {
        code: "SHP",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Saint Helenian Pound",
        symbol: "£",
        symbol_first: false,
    };
    pub const SKK: &'static Currency = &Currency {
        code: "SKK",
        exponent: 2,
        locale: EnUs,
        minor_units: 50,
        name: "Slovak Koruna",
        symbol: "Sk",
        symbol_first: true,
    };
    pub const SLE: &'static Currency = &Currency {
        code: "SLE",
        exponent: 2,
        locale: EnUs,
        minor_units: 1000,
        name: "Sierra Leonean Leone",
        symbol: "Le",
        symbol_first: false,
    };
    pub const SLL: &'static Currency = &Currency {
        code: "SLL",
        exponent: 2,
        locale: EnUs,
        minor_units: 1000,
        name: "Sierra Leonean Leone",
        symbol: "Le",
        symbol_first: false,
    };
    pub const SOS: &'static Currency = &Currency {
        code: "SOS",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Somali Shilling",
        symbol: "Sh",
        symbol_first: false,
    };
    pub const SRD: &'static Currency = &Currency {
        code: "SRD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Surinamese Dollar",
        symbol: "$",
        symbol_first: false,
    };
    pub const SSP: &'static Currency = &Currency {
        code: "SSP",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "South Sudanese Pound",
        symbol: "£",
        symbol_first: false,
    };
    pub const STD: &'static Currency = &Currency {
        code: "STD",
        exponent: 2,
        locale: EnUs,
        minor_units: 10000,
        name: "São Tomé and Príncipe Dobra",
        symbol: "Db",
        symbol_first: false,
    };
    pub const STN: &'static Currency = &Currency {
        code: "STN",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "São Tomé and Príncipe Dobra",
        symbol: "Db",
        symbol_first: false,
    };
    pub const SVC: &'static Currency = &Currency {
        code: "SVC",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Salvadoran Colón",
        symbol: "₡",
        symbol_first: true,
    };
    pub const SYP: &'static Currency = &Currency {
        code: "SYP",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Syrian Pound",
        symbol: "£S",
        symbol_first: false,
    };
    pub const SZL: &'static Currency = &Currency {
        code: "SZL",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Swazi Lilangeni",
        symbol: "E",
        symbol_first: true,
    };
    pub const THB: &'static Currency = &Currency {
        code: "THB",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Thai Baht",
        symbol: "฿",
        symbol_first: true,
    };
    pub const TJS: &'static Currency = &Currency {
        code: "TJS",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Tajikistani Somoni",
        symbol: "ЅМ",
        symbol_first: false,
    };
    pub const TMT: &'static Currency = &Currency {
        code: "TMT",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Turkmenistani Manat",
        symbol: "T",
        symbol_first: false,
    };
    pub const TND: &'static Currency = &Currency {
        code: "TND",
        exponent: 3,
        locale: EnUs,
        minor_units: 10,
        name: "Tunisian Dinar",
        symbol: "د.ت",
        symbol_first: false,
    };
    pub const TOP: &'static Currency = &Currency {
        code: "TOP",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Tongan Paʻanga",
        symbol: "T$",
        symbol_first: true,
    };
    pub const TRY: &'static Currency = &Currency {
        code: "TRY",
        exponent: 2,
        locale: EnEu,
        minor_units: 1,
        name: "Turkish Lira",
        symbol: "₺",
        symbol_first: true,
    };
    pub const TTD: &'static Currency = &Currency {
        code: "TTD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Trinidad and Tobago Dollar",
        symbol: "$",
        symbol_first: false,
    };
    pub const TWD: &'static Currency = &Currency {
        code: "TWD",
        exponent: 2,
        locale: EnUs,
        minor_units: 50,
        name: "New Taiwan Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const TZS: &'static Currency = &Currency {
        code: "TZS",
        exponent: 2,
        locale: EnUs,
        minor_units: 5000,
        name: "Tanzanian Shilling",
        symbol: "Sh",
        symbol_first: true,
    };
    pub const UAH: &'static Currency = &Currency {
        code: "UAH",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Ukrainian Hryvnia",
        symbol: "₴",
        symbol_first: false,
    };
    pub const UGX: &'static Currency = &Currency {
        code: "UGX",
        exponent: 0,
        locale: EnUs,
        minor_units: 1000,
        name: "Ugandan Shilling",
        symbol: "USh",
        symbol_first: false,
    };
    pub const USD: &'static Currency = &Currency {
        code: "USD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "United States Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const UYU: &'static Currency = &Currency {
        code: "UYU",
        exponent: 2,
        locale: EnEu,
        minor_units: 100,
        name: "Uruguayan Peso",
        symbol: "$U",
        symbol_first: true,
    };
    pub const UYW: &'static Currency = &Currency {
        code: "UYW",
        exponent: 4,
        locale: EnEu,
        minor_units: 1000,
        name: "Unidad Previsional",
        symbol: "UP",
        symbol_first: true,
    };
    pub const UZS: &'static Currency = &Currency {
        code: "UZS",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Uzbekistan Som",
        symbol: "so'm",
        symbol_first: false,
    };
    pub const VES: &'static Currency = &Currency {
        code: "VES",
        exponent: 2,
        locale: EnEu,
        minor_units: 1,
        name: "Venezuelan Bolívar Soberano",
        symbol: "Bs",
        symbol_first: true,
    };
    pub const VND: &'static Currency = &Currency {
        code: "VND",
        exponent: 0,
        locale: EnEu,
        minor_units: 100,
        name: "Vietnamese Đồng",
        symbol: "₫",
        symbol_first: false,
    };
    pub const VUV: &'static Currency = &Currency {
        code: "VUV",
        exponent: 0,
        locale: EnUs,
        minor_units: 1,
        name: "Vanuatu Vatu",
        symbol: "Vt",
        symbol_first: true,
    };
    pub const WST: &'static Currency = &Currency {
        code: "WST",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "Samoan Tala",
        symbol: "T",
        symbol_first: false,
    };
    pub const XAF: &'static Currency = &Currency {
        code: "XAF",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Central African Cfa Franc",
        symbol: "CFA",
        symbol_first: false,
    };
    pub const XAG: &'static Currency = &Currency {
        code: "XAG",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Silver (Troy Ounce)",
        symbol: "oz t",
        symbol_first: false,
    };
    pub const XAU: &'static Currency = &Currency {
        code: "XAU",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Gold (Troy Ounce)",
        symbol: "oz t",
        symbol_first: false,
    };
    pub const XBA: &'static Currency = &Currency {
        code: "XBA",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "European Composite Unit",
        symbol: "",
        symbol_first: false,
    };
    pub const XBB: &'static Currency = &Currency {
        code: "XBB",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "European Monetary Unit",
        symbol: "",
        symbol_first: false,
    };
    pub const XBC: &'static Currency = &Currency {
        code: "XBC",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "European Unit of Account 9",
        symbol: "",
        symbol_first: false,
    };
    pub const XBD: &'static Currency = &Currency {
        code: "XBD",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "European Unit of Account 17",
        symbol: "",
        symbol_first: false,
    };
    pub const XCD: &'static Currency = &Currency {
        code: "XCD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "East Caribbean Dollar",
        symbol: "$",
        symbol_first: true,
    };
    pub const XDR: &'static Currency = &Currency {
        code: "XDR",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Special Drawing Rights",
        symbol: "SDR",
        symbol_first: false,
    };
    pub const XOF: &'static Currency = &Currency {
        code: "XOF",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "West African Cfa Franc",
        symbol: "Fr",
        symbol_first: false,
    };
    pub const XPD: &'static Currency = &Currency {
        code: "XPD",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Palladium",
        symbol: "oz t",
        symbol_first: false,
    };
    pub const XPF: &'static Currency = &Currency {
        code: "XPF",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Cfp Franc",
        symbol: "Fr",
        symbol_first: false,
    };
    pub const XPT: &'static Currency = &Currency {
        code: "XPT",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Platinum",
        symbol: "oz t",
        symbol_first: false,
    };
    pub const XTS: &'static Currency = &Currency {
        code: "XTS",
        exponent: 0,
        locale: EnUs,
        minor_units: 100,
        name: "Codes specifically reserved for testing purposes",
        symbol: "oz t",
        symbol_first: false,
    };
    pub const YER: &'static Currency = &Currency {
        code: "YER",
        exponent: 2,
        locale: EnUs,
        minor_units: 100,
        name: "Yemeni Rial",
        symbol: "﷼",
        symbol_first: false,
    };
    pub const ZAR: &'static Currency = &Currency {
        code: "ZAR",
        exponent: 2,
        locale: EnUs,
        minor_units: 10,
        name: "South African Rand",
        symbol: "R",
        symbol_first: true,
    };
    pub const ZMK: &'static Currency = &Currency {
        code: "ZMK",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Zambian Kwacha",
        symbol: "ZK",
        symbol_first: false,
    };
    pub const ZMW: &'static Currency = &Currency {
        code: "ZMW",
        exponent: 2,
        locale: EnUs,
        minor_units: 5,
        name: "Zambian Kwacha",
        symbol: "K",
        symbol_first: true,
    };
    pub const ZWL: &'static Currency = &Currency {
        code: "ZWL",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "Zimbabwe Dollar",
        symbol: "Z$",
        symbol_first: true,
    };
}


#[cfg(test)]
mod tests {
    use crate::{find_currency, iso};

    #[test]
    fn find_returns_known_currencies() {
        let currency_by_alpha = find_currency("USD").unwrap();
        assert_eq!(currency_by_alpha.code, "USD");
        assert_eq!(currency_by_alpha.exponent, 2);
        assert_eq!(currency_by_alpha.symbol, "$");
    }

    #[test]
    fn find_raises_invalid_currency_error_on_unknown_currency() {
        assert_eq!(find_currency("fake"), None,);
    }

    #[test]
    fn currency_can_be_accessed_by_reference() {
        assert_eq!(iso::USD.code, "USD");
        assert_eq!(iso::USD.exponent, 2);
        assert_eq!(iso::USD.symbol, "$");
    }

    #[test]
    fn find_and_reference_point_to_same() {
        assert_eq!(iso::USD, find_currency("USD").unwrap());
    }
}
