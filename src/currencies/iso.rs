use crate::Currency;
use crate::Locale::{EnBy, EnEu, EnIn, EnUs};

pub const AED: Currency = Currency {
    exponent: 2,
    code: "AED",
    locale: EnUs,
    minor_units: 25,
    name: "United Arab Emirates Dirham",
    symbol: "د.إ",
    symbol_first: false,
};
pub const AFN: Currency = Currency {
    exponent: 2,
    code: "AFN",
    locale: EnUs,
    minor_units: 100,
    name: "Afghan Afghani",
    symbol: "؋",
    symbol_first: false,
};
pub const ALL: Currency = Currency {
    exponent: 2,
    code: "ALL",
    locale: EnEu,
    minor_units: 1,
    name: "Albanian lek",
    symbol: "L",
    symbol_first: false,
};
pub const AMD: Currency = Currency {
    exponent: 2,
    code: "AMD",
    locale: EnUs,
    minor_units: 10,
    name: "Armenian Dram",
    symbol: "դր.",
    symbol_first: false,
};
pub const ANG: Currency = Currency {
    exponent: 2,
    code: "ANG",
    locale: EnUs,
    minor_units: 1,
    name: "Netherlands Antillean Gulden",
    symbol: "ƒ",
    symbol_first: false,
};
pub const AOA: Currency = Currency {
    exponent: 2,
    code: "AOA",
    locale: EnUs,
    minor_units: 10,
    name: "Angolan Kwanza",
    symbol: "Kz",
    symbol_first: false,
};
pub const ARS: Currency = Currency {
    exponent: 2,
    code: "ARS",
    locale: EnEu,
    minor_units: 1,
    name: "Argentine Peso",
    symbol: "$",
    symbol_first: true,
};
pub const AUD: Currency = Currency {
    exponent: 2,
    code: "AUD",
    locale: EnUs,
    minor_units: 5,
    name: "Australian Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const AWG: Currency = Currency {
    exponent: 2,
    code: "AWG",
    locale: EnUs,
    minor_units: 5,
    name: "Aruban Florin",
    symbol: "ƒ",
    symbol_first: false,
};
pub const AZN: Currency = Currency {
    exponent: 2,
    code: "AZN",
    locale: EnUs,
    minor_units: 1,
    name: "Azerbaijani Manat",
    symbol: "₼",
    symbol_first: true,
};
pub const BAM: Currency = Currency {
    exponent: 2,
    code: "BAM",
    locale: EnUs,
    minor_units: 5,
    name: "Bosnia and Herzegovina Convertible Mark",
    symbol: "KM",
    symbol_first: true,
};
pub const BBD: Currency = Currency {
    exponent: 2,
    code: "BBD",
    locale: EnUs,
    minor_units: 1,
    name: "Barbadian Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const BDT: Currency = Currency {
    exponent: 2,
    code: "BDT",
    locale: EnIn,
    minor_units: 1,
    name: "Bangladeshi Taka",
    symbol: "৳",
    symbol_first: true,
};
pub const BGN: Currency = Currency {
    exponent: 2,
    code: "BGN",
    locale: EnIn,
    minor_units: 1,
    name: "Bulgarian Lev",
    symbol: "лв.",
    symbol_first: false,
};
pub const BHD: Currency = Currency {
    exponent: 3,
    code: "BHD",
    locale: EnUs,
    minor_units: 5,
    name: "Bahraini Dinar",
    symbol: "د.ب",
    symbol_first: true,
};
pub const BIF: Currency = Currency {
    exponent: 0,
    code: "BIF",
    locale: EnUs,
    minor_units: 100,
    name: "Burundian Franc",
    symbol: "Fr",
    symbol_first: false,
};
pub const BMD: Currency = Currency {
    exponent: 2,
    code: "BMD",
    locale: EnUs,
    minor_units: 1,
    name: "Bermudian Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const BND: Currency = Currency {
    exponent: 2,
    code: "BND",
    locale: EnUs,
    minor_units: 1,
    name: "Brunei Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const BOB: Currency = Currency {
    exponent: 2,
    code: "BOB",
    locale: EnUs,
    minor_units: 10,
    name: "Bolivian Boliviano",
    symbol: "Bs.",
    symbol_first: true,
};
pub const BRL: Currency = Currency {
    exponent: 2,
    code: "BRL",
    locale: EnUs,
    minor_units: 5,
    name: "Brazilian real",
    symbol: "R$",
    symbol_first: true,
};
pub const BSD: Currency = Currency {
    exponent: 2,
    code: "BSD",
    locale: EnUs,
    minor_units: 1,
    name: "Bahamian Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const BTN: Currency = Currency {
    exponent: 2,
    code: "BTN",
    locale: EnUs,
    minor_units: 5,
    name: "Bhutanese Ngultrum",
    symbol: "Nu.",
    symbol_first: false,
};
pub const BWP: Currency = Currency {
    exponent: 2,
    code: "BWP",
    locale: EnUs,
    minor_units: 5,
    name: "Botswana Pula",
    symbol: "P",
    symbol_first: true,
};
pub const BYN: Currency = Currency {
    exponent: 2,
    code: "BYN",
    locale: EnBy,
    minor_units: 1,
    name: "Belarusian Ruble",
    symbol: "Br",
    symbol_first: false,
};
pub const BYR: Currency = Currency {
    exponent: 0,
    code: "BYR",
    locale: EnBy,
    minor_units: 100,
    name: "Belarusian Ruble",
    symbol: "Br",
    symbol_first: false,
};
pub const BZD: Currency = Currency {
    exponent: 2,
    code: "BZD",
    locale: EnUs,
    minor_units: 1,
    name: "Belize Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const CAD: Currency = Currency {
    exponent: 2,
    code: "CAD",
    locale: EnUs,
    minor_units: 5,
    name: "Canadian Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const CDF: Currency = Currency {
    exponent: 2,
    code: "CDF",
    locale: EnUs,
    minor_units: 1,
    name: "Congolese Franc",
    symbol: "Fr",
    symbol_first: false,
};
pub const CHF: Currency = Currency {
    exponent: 2,
    code: "CHF",
    locale: EnUs,
    minor_units: 5,
    name: "Swiss Franc",
    symbol: "Fr",
    symbol_first: true,
};
pub const CLF: Currency = Currency {
    exponent: 4,
    code: "CLF",
    locale: EnEu,
    minor_units: 5,
    name: "Unidad de Fomento",
    symbol: "UF",
    symbol_first: true,
};
pub const CLP: Currency = Currency {
    exponent: 0,
    code: "CLP",
    locale: EnEu,
    minor_units: 1,
    name: "Chilean Peso",
    symbol: "$",
    symbol_first: true,
};
pub const CNY: Currency = Currency {
    exponent: 2,
    code: "CNY",
    locale: EnUs,
    minor_units: 1,
    name: "Chinese Renminbi Yuan",
    symbol: "¥",
    symbol_first: true,
};
pub const COP: Currency = Currency {
    exponent: 2,
    code: "COP",
    locale: EnEu,
    minor_units: 20,
    name: "Colombian Peso",
    symbol: "$",
    symbol_first: true,
};
pub const CRC: Currency = Currency {
    exponent: 2,
    code: "CRC",
    locale: EnEu,
    minor_units: 500, // TODO - Investigate
    name: "Costa Rican Colón",
    symbol: "₡",
    symbol_first: true,
};
pub const CUC: Currency = Currency {
    exponent: 2,
    code: "CUC",
    locale: EnUs,
    minor_units: 1,
    name: "Cuban Convertible Peso",
    symbol: "$",
    symbol_first: false,
};
pub const CUP: Currency = Currency {
    exponent: 2,
    code: "CUP",
    locale: EnUs,
    minor_units: 1,
    name: "Cuban Peso",
    symbol: "$",
    symbol_first: true,
};
pub const CVE: Currency = Currency {
    exponent: 2,
    code: "CVE",
    locale: EnUs,
    minor_units: 100,
    name: "Cape Verdean Escudo",
    symbol: "$",
    symbol_first: false,
};
pub const CZK: Currency = Currency {
    exponent: 2,
    code: "CZK",
    locale: EnBy,
    minor_units: 100,
    name: "Czech Koruna",
    symbol: "Kč",
    symbol_first: false,
};
pub const DJF: Currency = Currency {
    exponent: 0,
    code: "DJF",
    locale: EnUs,
    minor_units: 100,
    name: "Djiboutian Franc",
    symbol: "Fdj",
    symbol_first: false,
};
pub const DKK: Currency = Currency {
    exponent: 2,
    code: "DKK",
    locale: EnEu,
    minor_units: 50,
    name: "Danish Krone",
    symbol: "kr.",
    symbol_first: false,
};
pub const DOP: Currency = Currency {
    exponent: 2,
    code: "DOP",
    locale: EnUs,
    minor_units: 100,
    name: "Dominican Peso",
    symbol: "$",
    symbol_first: true,
};
pub const DZD: Currency = Currency {
    exponent: 2,
    code: "DZD",
    locale: EnUs,
    minor_units: 100,
    name: "Algerian Dinar",
    symbol: "د.ج",
    symbol_first: false,
};
pub const EGP: Currency = Currency {
    exponent: 2,
    code: "EGP",
    locale: EnUs,
    minor_units: 25,
    name: "Egyptian Pound",
    symbol: "ج.م",
    symbol_first: true,
};
pub const ERN: Currency = Currency {
    exponent: 2,
    code: "ERN",
    locale: EnUs,
    minor_units: 1,
    name: "Eritrean Nakfa",
    symbol: "Nfk",
    symbol_first: false,
};
pub const ETB: Currency = Currency {
    exponent: 2,
    code: "ETB",
    locale: EnUs,
    minor_units: 1,
    name: "Ethiopian Birr",
    symbol: "Br",
    symbol_first: false,
};
pub const EUR: Currency = Currency {
    exponent: 2,
    code: "EUR",
    locale: EnEu,
    minor_units: 1,
    name: "Euro",
    symbol: "€",
    symbol_first: true,
};
pub const FJD: Currency = Currency {
    exponent: 2,
    code: "FJD",
    locale: EnEu,
    minor_units: 5,
    name: "Fijian Dollar",
    symbol: "$",
    symbol_first: false,
};
pub const FKP: Currency = Currency {
    exponent: 2,
    code: "FKP",
    locale: EnEu,
    minor_units: 1,
    name: "Falkland Pound",
    symbol: "£",
    symbol_first: false,
};
pub const GBP: Currency = Currency {
    exponent: 2,
    code: "GBP",
    locale: EnUs,
    minor_units: 1,
    name: "British Pound",
    symbol: "£",
    symbol_first: true,
};
pub const GEL: Currency = Currency {
    exponent: 2,
    code: "GEL",
    locale: EnUs,
    minor_units: 1,
    name: "Georgian Lari",
    symbol: "ლ",
    symbol_first: false,
};
pub const GHS: Currency = Currency {
    exponent: 2,
    code: "GHS",
    locale: EnUs,
    minor_units: 1,
    name: "Ghanaian Cedi",
    symbol: "₵",
    symbol_first: true,
};
pub const GIP: Currency = Currency {
    exponent: 2,
    code: "GIP",
    locale: EnUs,
    minor_units: 1,
    name: "Gibraltar Pound",
    symbol: "£",
    symbol_first: true,
};
pub const GMD: Currency = Currency {
    exponent: 2,
    code: "GMD",
    locale: EnUs,
    minor_units: 1,
    name: "Gambian Dalasi",
    symbol: "D",
    symbol_first: false,
};
pub const GNF: Currency = Currency {
    exponent: 0,
    code: "GNF",
    locale: EnUs,
    minor_units: 100,
    name: "Guinean Franc",
    symbol: "Fr",
    symbol_first: false,
};
pub const GTQ: Currency = Currency {
    exponent: 2,
    code: "GTQ",
    locale: EnUs,
    minor_units: 1,
    name: "Guatemalan Quetzal",
    symbol: "Q",
    symbol_first: true,
};
pub const GYD: Currency = Currency {
    exponent: 2,
    code: "GYD",
    locale: EnUs,
    minor_units: 100,
    name: "Guyanese Dollar",
    symbol: "$",
    symbol_first: false,
};
pub const HKD: Currency = Currency {
    exponent: 2,
    code: "HKD",
    locale: EnUs,
    minor_units: 10,
    name: "Hong Kong Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const HNL: Currency = Currency {
    exponent: 2,
    code: "HNL",
    locale: EnUs,
    minor_units: 5,
    name: "Honduran Lempira",
    symbol: "L",
    symbol_first: true,
};
pub const HRK: Currency = Currency {
    exponent: 2,
    code: "HRK",
    locale: EnEu,
    minor_units: 1,
    name: "Croatian Kuna",
    symbol: "kn",
    symbol_first: false,
};
pub const HTG: Currency = Currency {
    exponent: 2,
    code: "HTG",
    locale: EnUs,
    minor_units: 5,
    name: "Haitian Gourde",
    symbol: "G",
    symbol_first: false,
};
pub const HUF: Currency = Currency {
    exponent: 0,
    code: "HUF",
    locale: EnBy,
    minor_units: 5,
    name: "Hungarian Forint",
    symbol: "Ft",
    symbol_first: false,
};
pub const IDR: Currency = Currency {
    exponent: 2,
    code: "IDR",
    locale: EnUs,
    minor_units: 5000,
    name: "Indonesian Rupiah",
    symbol: "Rp",
    symbol_first: true,
};
pub const ILS: Currency = Currency {
    exponent: 2,
    code: "ILS",
    locale: EnUs,
    minor_units: 10,
    name: "Israeli New Sheqel",
    symbol: "₪",
    symbol_first: true,
};
pub const INR: Currency = Currency {
    exponent: 2,
    code: "INR",
    locale: EnIn,
    minor_units: 50,
    name: "Indian Rupee",
    symbol: "₹",
    symbol_first: true,
};
pub const IQD: Currency = Currency {
    exponent: 3,
    code: "IQD",
    locale: EnUs,
    minor_units: 50000,
    name: "Iraqi Dinar",
    symbol: "ع.د",
    symbol_first: false,
};
pub const IRR: Currency = Currency {
    exponent: 2,
    code: "IRR",
    locale: EnUs,
    minor_units: 5000,
    name: "Iranian Rial",
    symbol: "﷼",
    symbol_first: true,
};
pub const ISK: Currency = Currency {
    exponent: 0,
    code: "ISK",
    locale: EnEu,
    minor_units: 1,
    name: "Icelandic Króna",
    symbol: "kr.",
    symbol_first: true,
};
pub const JMD: Currency = Currency {
    exponent: 2,
    code: "JMD",
    locale: EnUs,
    minor_units: 1,
    name: "Jamaican Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const JOD: Currency = Currency {
    exponent: 3,
    code: "JOD",
    locale: EnUs,
    minor_units: 5,
    name: "Jordanian Dinar",
    symbol: "د.ا",
    symbol_first: true,
};
pub const JPY: Currency = Currency {
    exponent: 0,
    code: "JPY",
    locale: EnUs,
    minor_units: 1,
    name: "Japanese Yen",
    symbol: "¥",
    symbol_first: true,
};
pub const KES: Currency = Currency {
    exponent: 2,
    code: "KES",
    locale: EnUs,
    minor_units: 50,
    name: "Kenyan Shilling",
    symbol: "KSh",
    symbol_first: true,
};
pub const KGS: Currency = Currency {
    exponent: 2,
    code: "KGS",
    locale: EnUs,
    minor_units: 1,
    name: "Kyrgyzstani Som",
    symbol: "som",
    symbol_first: false,
};
pub const KHR: Currency = Currency {
    exponent: 2,
    code: "KHR",
    locale: EnUs,
    minor_units: 5000,
    name: "Cambodian Riel",
    symbol: "៛",
    symbol_first: false,
};
pub const KMF: Currency = Currency {
    exponent: 0,
    code: "KMF",
    locale: EnUs,
    minor_units: 100,
    name: "Comorian Franc",
    symbol: "Fr",
    symbol_first: false,
};
pub const KPW: Currency = Currency {
    exponent: 2,
    code: "KPW",
    locale: EnUs,
    minor_units: 1,
    name: "North Korean Won",
    symbol: "₩",
    symbol_first: false,
};
pub const KRW: Currency = Currency {
    exponent: 0,
    code: "KRW",
    locale: EnUs,
    minor_units: 1,
    name: "South Korean Won",
    symbol: "₩",
    symbol_first: true,
};
pub const KWD: Currency = Currency {
    exponent: 3,
    code: "KWD",
    locale: EnUs,
    minor_units: 5,
    name: "Kuwaiti Dinar",
    symbol: "د.ك",
    symbol_first: true,
};
pub const KYD: Currency = Currency {
    exponent: 2,
    code: "KYD",
    locale: EnUs,
    minor_units: 1,
    name: "Cayman Islands Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const KZT: Currency = Currency {
    exponent: 2,
    code: "KZT",
    locale: EnUs,
    minor_units: 100,
    name: "Kazakhstani Tenge",
    symbol: "₸",
    symbol_first: false,
};
pub const LAK: Currency = Currency {
    exponent: 2,
    code: "LAK",
    locale: EnUs,
    minor_units: 10,
    name: "Lao Kip",
    symbol: "₭",
    symbol_first: false,
};
pub const LBP: Currency = Currency {
    exponent: 2,
    code: "LBP",
    locale: EnUs,
    minor_units: 25000,
    name: "Lebanese Pound",
    symbol: "ل.ل",
    symbol_first: true,
};
pub const LKR: Currency = Currency {
    exponent: 2,
    code: "LKR",
    locale: EnUs,
    minor_units: 100,
    name: "Sri Lankan Rupee",
    symbol: "₨",
    symbol_first: false,
};
pub const LRD: Currency = Currency {
    exponent: 2,
    code: "LRD",
    locale: EnUs,
    minor_units: 5,
    name: "Liberian Dollar",
    symbol: "$",
    symbol_first: false,
};
pub const LSL: Currency = Currency {
    exponent: 2,
    code: "LSL",
    locale: EnUs,
    minor_units: 1,
    name: "Lesotho Loti",
    symbol: "L",
    symbol_first: false,
};
pub const LYD: Currency = Currency {
    exponent: 3,
    code: "LYD",
    locale: EnUs,
    minor_units: 50,
    name: "Libyan Dinar",
    symbol: "ل.د",
    symbol_first: false,
};
pub const MAD: Currency = Currency {
    exponent: 2,
    code: "MAD",
    locale: EnUs,
    minor_units: 1,
    name: "Moroccan Dirham",
    symbol: "د.م.",
    symbol_first: false,
};
pub const MDL: Currency = Currency {
    exponent: 2,
    code: "MDL",
    locale: EnUs,
    minor_units: 1,
    name: "Moldovan Leu",
    symbol: "L",
    symbol_first: false,
};
pub const MGA: Currency = Currency {
    exponent: 1, // TODO - exponent is 1/5th need to represent somehow
    code: "MGA",
    locale: EnUs,
    minor_units: 1,
    name: "Malagasy Ariary",
    symbol: "Ar",
    symbol_first: true,
};
pub const MKD: Currency = Currency {
    exponent: 2,
    code: "MKD",
    locale: EnUs,
    minor_units: 100,
    name: "Macedonian Denar",
    symbol: "ден",
    symbol_first: false,
};
pub const MMK: Currency = Currency {
    exponent: 2,
    code: "MMK",
    locale: EnUs,
    minor_units: 50,
    name: "Myanmar Kyat",
    symbol: "K",
    symbol_first: false,
};
pub const MNT: Currency = Currency {
    exponent: 2,
    code: "MNT",
    locale: EnUs,
    minor_units: 2000,
    name: "Mongolian Tögrög",
    symbol: "₮",
    symbol_first: false,
};
pub const MOP: Currency = Currency {
    exponent: 2,
    code: "MOP",
    locale: EnUs,
    minor_units: 10,
    name: "Macanese Pataca",
    symbol: "P",
    symbol_first: false,
};
pub const MRU: Currency = Currency {
    exponent: 1, // TODO - exponent problem of 5
    code: "MRU",
    locale: EnUs,
    minor_units: 1,
    name: "Mauritanian Ouguiya",
    symbol: "UM",
    symbol_first: false,
};
pub const MUR: Currency = Currency {
    exponent: 2,
    code: "MUR",
    locale: EnUs,
    minor_units: 100,
    name: "Mauritian Rupee",
    symbol: "₨",
    symbol_first: true,
};
pub const MVR: Currency = Currency {
    exponent: 2,
    code: "MVR",
    locale: EnUs,
    minor_units: 100,
    name: "Maldivian Rufiyaa",
    symbol: "MVR",
    symbol_first: false,
};
pub const MWK: Currency = Currency {
    exponent: 2,
    code: "MWK",
    locale: EnUs,
    minor_units: 1,
    name: "Malawian Kwacha",
    symbol: "MK",
    symbol_first: false,
};
pub const MXN: Currency = Currency {
    exponent: 2,
    code: "MXN",
    locale: EnUs,
    minor_units: 5,
    name: "Mexican Peso",
    symbol: "$",
    symbol_first: true,
};
pub const MYR: Currency = Currency {
    exponent: 2,
    code: "MYR",
    locale: EnUs,
    minor_units: 5,
    name: "Malaysian Ringgit",
    symbol: "RM",
    symbol_first: true,
};
pub const MZN: Currency = Currency {
    exponent: 2,
    code: "MZN",
    locale: EnUs,
    minor_units: 1,
    name: "Mozambican Metical",
    symbol: "MTn",
    symbol_first: true,
};
pub const NAD: Currency = Currency {
    exponent: 2,
    code: "NAD",
    locale: EnUs,
    minor_units: 5,
    name: "Namibian Dollar",
    symbol: "$",
    symbol_first: false,
};
pub const NGN: Currency = Currency {
    exponent: 2,
    code: "NGN",
    locale: EnUs,
    minor_units: 50,
    name: "Nigerian Naira",
    symbol: "₦",
    symbol_first: true,
};
pub const NIO: Currency = Currency {
    exponent: 2,
    code: "NIO",
    locale: EnUs,
    minor_units: 5,
    name: "Nicaraguan Córdoba",
    symbol: "C$",
    symbol_first: true,
};
pub const NOK: Currency = Currency {
    exponent: 2,
    code: "NOK",
    locale: EnUs,
    minor_units: 100,
    name: "Norwegian Krone",
    symbol: "kr",
    symbol_first: false,
};
pub const NPR: Currency = Currency {
    exponent: 2,
    code: "NPR",
    locale: EnUs,
    minor_units: 1,
    name: "Nepalese Rupee",
    symbol: "रु",
    symbol_first: true,
};
pub const NZD: Currency = Currency {
    exponent: 2,
    code: "NZD",
    locale: EnUs,
    minor_units: 10,
    name: "New Zealand Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const OMR: Currency = Currency {
    exponent: 3,
    code: "OMR",
    locale: EnUs,
    minor_units: 5,
    name: "Omani Rial",
    symbol: "ر.ع.",
    symbol_first: true,
};
pub const PAB: Currency = Currency {
    exponent: 2,
    code: "PAB",
    locale: EnUs,
    minor_units: 1,
    name: "Panamanian Balboa",
    symbol: "B/.",
    symbol_first: true,
};
pub const PEN: Currency = Currency {
    exponent: 2,
    code: "PEN",
    locale: EnUs,
    minor_units: 1,
    name: "Peruvian Sol",
    symbol: "S/",
    symbol_first: true,
};
pub const PGK: Currency = Currency {
    exponent: 2,
    code: "PGK",
    locale: EnUs,
    minor_units: 5,
    name: "Papua New Guinean Kina",
    symbol: "K",
    symbol_first: false,
};
pub const PHP: Currency = Currency {
    exponent: 2,
    code: "PHP",
    locale: EnUs,
    minor_units: 1,
    name: "Philippine Peso",
    symbol: "₱",
    symbol_first: true,
};
pub const PKR: Currency = Currency {
    exponent: 2,
    code: "PKR",
    locale: EnUs,
    minor_units: 100,
    name: "Pakistani Rupee",
    symbol: "₨",
    symbol_first: true,
};
pub const PLN: Currency = Currency {
    exponent: 2,
    code: "PLN",
    locale: EnBy,
    minor_units: 1,
    name: "Polish Złoty",
    symbol: "zł",
    symbol_first: false,
};
pub const PYG: Currency = Currency {
    exponent: 0,
    code: "PYG",
    locale: EnBy,
    minor_units: 5000,
    name: "Paraguayan Guaraní",
    symbol: "₲",
    symbol_first: true,
};
pub const QAR: Currency = Currency {
    exponent: 2,
    code: "QAR",
    locale: EnBy,
    minor_units: 1,
    name: "Qatari Riyal",
    symbol: "ر.ق",
    symbol_first: false,
};
pub const RON: Currency = Currency {
    exponent: 2,
    code: "RON",
    locale: EnEu,
    minor_units: 1,
    name: "Romanian Leu",
    symbol: "RON",
    symbol_first: false,
};
pub const ROL: Currency = Currency {
    exponent: 0,
    code: "ROL",
    locale: EnEu,
    minor_units: 1,
    name: "Romanian Leu",
    symbol: "ROL",
    symbol_first: false,
};
pub const RSD: Currency = Currency {
    exponent: 2,
    code: "RSD",
    locale: EnUs,
    minor_units: 100,
    name: "Serbian Dinar",
    symbol: "РСД",
    symbol_first: true,
};
pub const RUB: Currency = Currency {
    exponent: 2,
    code: "RUB",
    locale: EnEu,
    minor_units: 1,
    name: "Russian Ruble",
    symbol: "₽",
    symbol_first: false,
};
pub const RWF: Currency = Currency {
    exponent: 0,
    code: "RWF",
    locale: EnUs,
    minor_units: 100,
    name: "Rwandan Franc",
    symbol: "FRw",
    symbol_first: false,
};
pub const SAR: Currency = Currency {
    exponent: 2,
    code: "SAR",
    locale: EnUs,
    minor_units: 5,
    name: "Saudi Riyal",
    symbol: "ر.س",
    symbol_first: true,
};
pub const SBD: Currency = Currency {
    exponent: 2,
    code: "SBD",
    locale: EnUs,
    minor_units: 10,
    name: "Solomon Islands Dollar",
    symbol: "$",
    symbol_first: false,
};
pub const SCR: Currency = Currency {
    exponent: 2,
    code: "SCR",
    locale: EnUs,
    minor_units: 1,
    name: "Seychellois Rupee",
    symbol: "₨",
    symbol_first: false,
};
pub const SDG: Currency = Currency {
    exponent: 2,
    code: "SDG",
    locale: EnUs,
    minor_units: 1,
    name: "Sudanese Pound",
    symbol: "£",
    symbol_first: true,
};
pub const SEK: Currency = Currency {
    exponent: 2,
    code: "SEK",
    locale: EnBy,
    minor_units: 100,
    name: "Swedish Krona",
    symbol: "kr",
    symbol_first: false,
};
pub const SGD: Currency = Currency {
    exponent: 2,
    code: "SGD",
    locale: EnUs,
    minor_units: 1,
    name: "Singapore Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const SHP: Currency = Currency {
    exponent: 2,
    code: "SHP",
    locale: EnUs,
    minor_units: 1,
    name: "Saint Helenian Pound",
    symbol: "£",
    symbol_first: false,
};
pub const SKK: Currency = Currency {
    exponent: 2,
    code: "SKK",
    locale: EnUs,
    minor_units: 50,
    name: "Slovak Koruna",
    symbol: "Sk",
    symbol_first: true,
};
pub const SLE: Currency = Currency {
    exponent: 2,
    code: "SLE",
    locale: EnUs,
    minor_units: 1000,
    name: "Sierra Leonean Leone",
    symbol: "Le",
    symbol_first: false,
};
pub const SLL: Currency = Currency {
    exponent: 2,
    code: "SLL",
    locale: EnUs,
    minor_units: 1000,
    name: "Sierra Leonean Leone",
    symbol: "Le",
    symbol_first: false,
};
pub const SOS: Currency = Currency {
    exponent: 2,
    code: "SOS",
    locale: EnUs,
    minor_units: 1,
    name: "Somali Shilling",
    symbol: "Sh",
    symbol_first: false,
};
pub const SRD: Currency = Currency {
    exponent: 2,
    code: "SRD",
    locale: EnUs,
    minor_units: 1,
    name: "Surinamese Dollar",
    symbol: "$",
    symbol_first: false,
};
pub const SSP: Currency = Currency {
    exponent: 2,
    code: "SSP",
    locale: EnUs,
    minor_units: 5,
    name: "South Sudanese Pound",
    symbol: "£",
    symbol_first: false,
};
pub const STD: Currency = Currency {
    exponent: 2,
    code: "STD",
    locale: EnUs,
    minor_units: 10000,
    name: "São Tomé and Príncipe Dobra",
    symbol: "Db",
    symbol_first: false,
};
pub const STN: Currency = Currency {
    exponent: 2,
    code: "STN",
    locale: EnUs,
    minor_units: 10,
    name: "São Tomé and Príncipe Dobra",
    symbol: "Db",
    symbol_first: false,
};
pub const SVC: Currency = Currency {
    exponent: 2,
    code: "SVC",
    locale: EnUs,
    minor_units: 1,
    name: "Salvadoran Colón",
    symbol: "₡",
    symbol_first: true,
};
pub const SYP: Currency = Currency {
    exponent: 2,
    code: "SYP",
    locale: EnUs,
    minor_units: 100,
    name: "Syrian Pound",
    symbol: "£S",
    symbol_first: false,
};
pub const SZL: Currency = Currency {
    exponent: 2,
    code: "SZL",
    locale: EnUs,
    minor_units: 1,
    name: "Swazi Lilangeni",
    symbol: "E",
    symbol_first: true,
};
pub const THB: Currency = Currency {
    exponent: 2,
    code: "THB",
    locale: EnUs,
    minor_units: 1,
    name: "Thai Baht",
    symbol: "฿",
    symbol_first: true,
};
pub const TJS: Currency = Currency {
    exponent: 2,
    code: "TJS",
    locale: EnUs,
    minor_units: 1,
    name: "Tajikistani Somoni",
    symbol: "ЅМ",
    symbol_first: false,
};
pub const TMT: Currency = Currency {
    exponent: 2,
    code: "TMT",
    locale: EnUs,
    minor_units: 1,
    name: "Turkmenistani Manat",
    symbol: "T",
    symbol_first: false,
};
pub const TND: Currency = Currency {
    exponent: 3,
    code: "TND",
    locale: EnUs,
    minor_units: 10,
    name: "Tunisian Dinar",
    symbol: "د.ت",
    symbol_first: false,
};
pub const TOP: Currency = Currency {
    exponent: 2,
    code: "TOP",
    locale: EnUs,
    minor_units: 1,
    name: "Tongan Paʻanga",
    symbol: "T$",
    symbol_first: true,
};
pub const TRY: Currency = Currency {
    exponent: 2,
    code: "TRY",
    locale: EnEu,
    minor_units: 1,
    name: "Turkish Lira",
    symbol: "₺",
    symbol_first: true,
};
pub const TTD: Currency = Currency {
    exponent: 2,
    code: "TTD",
    locale: EnUs,
    minor_units: 1,
    name: "Trinidad and Tobago Dollar",
    symbol: "$",
    symbol_first: false,
};
pub const TWD: Currency = Currency {
    exponent: 2,
    code: "TWD",
    locale: EnUs,
    minor_units: 50,
    name: "New Taiwan Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const TZS: Currency = Currency {
    exponent: 2,
    code: "TZS",
    locale: EnUs,
    minor_units: 5000,
    name: "Tanzanian Shilling",
    symbol: "Sh",
    symbol_first: true,
};
pub const UAH: Currency = Currency {
    exponent: 2,
    code: "UAH",
    locale: EnUs,
    minor_units: 1,
    name: "Ukrainian Hryvnia",
    symbol: "₴",
    symbol_first: false,
};
pub const UGX: Currency = Currency {
    exponent: 0,
    code: "UGX",
    locale: EnUs,
    minor_units: 1000,
    name: "Ugandan Shilling",
    symbol: "USh",
    symbol_first: false,
};
pub const USD: Currency = Currency {
    exponent: 2,
    code: "USD",
    locale: EnUs,
    minor_units: 1,
    name: "United States Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const UYU: Currency = Currency {
    exponent: 2,
    code: "UYU",
    locale: EnEu,
    minor_units: 100,
    name: "Uruguayan Peso",
    symbol: "$U",
    symbol_first: true,
};
pub const UYW: Currency = Currency {
    exponent: 4,
    code: "UYW",
    locale: EnEu,
    minor_units: 1000,
    name: "Unidad Previsional",
    symbol: "UP",
    symbol_first: true,
};
pub const UZS: Currency = Currency {
    exponent: 2,
    code: "UZS",
    locale: EnUs,
    minor_units: 100,
    name: "Uzbekistan Som",
    symbol: "so'm",
    symbol_first: false,
};
pub const VES: Currency = Currency {
    exponent: 2,
    code: "VES",
    locale: EnEu,
    minor_units: 1,
    name: "Venezuelan Bolívar Soberano",
    symbol: "Bs",
    symbol_first: true,
};
pub const VND: Currency = Currency {
    exponent: 0,
    code: "VND",
    locale: EnEu,
    minor_units: 100,
    name: "Vietnamese Đồng",
    symbol: "₫",
    symbol_first: false,
};
pub const VUV: Currency = Currency {
    exponent: 0,
    code: "VUV",
    locale: EnUs,
    minor_units: 1,
    name: "Vanuatu Vatu",
    symbol: "Vt",
    symbol_first: true,
};
pub const WST: Currency = Currency {
    exponent: 2,
    code: "WST",
    locale: EnUs,
    minor_units: 10,
    name: "Samoan Tala",
    symbol: "T",
    symbol_first: false,
};
pub const XAF: Currency = Currency {
    exponent: 0,
    code: "XAF",
    locale: EnUs,
    minor_units: 100,
    name: "Central African Cfa Franc",
    symbol: "CFA",
    symbol_first: false,
};
pub const XAG: Currency = Currency {
    exponent: 0,
    code: "XAG",
    locale: EnUs,
    minor_units: 100,
    name: "Silver (Troy Ounce)",
    symbol: "oz t",
    symbol_first: false,
};
pub const XAU: Currency = Currency {
    exponent: 0,
    code: "XAU",
    locale: EnUs,
    minor_units: 100,
    name: "Gold (Troy Ounce)",
    symbol: "oz t",
    symbol_first: false,
};
pub const XBA: Currency = Currency {
    exponent: 0,
    code: "XBA",
    locale: EnUs,
    minor_units: 100,
    name: "European Composite Unit",
    symbol: "",
    symbol_first: false,
};
pub const XBB: Currency = Currency {
    exponent: 0,
    code: "XBB",
    locale: EnUs,
    minor_units: 100,
    name: "European Monetary Unit",
    symbol: "",
    symbol_first: false,
};
pub const XBC: Currency = Currency {
    exponent: 0,
    code: "XBC",
    locale: EnUs,
    minor_units: 100,
    name: "European Unit of Account 9",
    symbol: "",
    symbol_first: false,
};
pub const XBD: Currency = Currency {
    exponent: 0,
    code: "XBD",
    locale: EnUs,
    minor_units: 100,
    name: "European Unit of Account 17",
    symbol: "",
    symbol_first: false,
};
pub const XCD: Currency = Currency {
    exponent: 2,
    code: "XCD",
    locale: EnUs,
    minor_units: 1,
    name: "East Caribbean Dollar",
    symbol: "$",
    symbol_first: true,
};
pub const XDR: Currency = Currency {
    exponent: 0,
    code: "XDR",
    locale: EnUs,
    minor_units: 100,
    name: "Special Drawing Rights",
    symbol: "SDR",
    symbol_first: false,
};
pub const XOF: Currency = Currency {
    exponent: 0,
    code: "XOF",
    locale: EnUs,
    minor_units: 100,
    name: "West African Cfa Franc",
    symbol: "Fr",
    symbol_first: false,
};
pub const XPD: Currency = Currency {
    exponent: 0,
    code: "XPD",
    locale: EnUs,
    minor_units: 100,
    name: "Palladium",
    symbol: "oz t",
    symbol_first: false,
};
pub const XPF: Currency = Currency {
    exponent: 0,
    code: "XPF",
    locale: EnUs,
    minor_units: 100,
    name: "Cfp Franc",
    symbol: "Fr",
    symbol_first: false,
};
pub const XPT: Currency = Currency {
    exponent: 0,
    code: "XPT",
    locale: EnUs,
    minor_units: 100,
    name: "Platinum",
    symbol: "oz t",
    symbol_first: false,
};
pub const XTS: Currency = Currency {
    exponent: 0,
    code: "XTS",
    locale: EnUs,
    minor_units: 100,
    name: "Codes specifically reserved for testing purposes",
    symbol: "oz t",
    symbol_first: false,
};
pub const YER: Currency = Currency {
    exponent: 2,
    code: "YER",
    locale: EnUs,
    minor_units: 100,
    name: "Yemeni Rial",
    symbol: "﷼",
    symbol_first: false,
};
pub const ZAR: Currency = Currency {
    exponent: 2,
    code: "ZAR",
    locale: EnUs,
    minor_units: 10,
    name: "South African Rand",
    symbol: "R",
    symbol_first: true,
};
pub const ZMK: Currency = Currency {
    exponent: 2,
    code: "ZMK",
    locale: EnUs,
    minor_units: 5,
    name: "Zambian Kwacha",
    symbol: "ZK",
    symbol_first: false,
};
pub const ZMW: Currency = Currency {
    exponent: 2,
    code: "ZMW",
    locale: EnUs,
    minor_units: 5,
    name: "Zambian Kwacha",
    symbol: "K",
    symbol_first: true,
};
pub const ZWL: Currency = Currency {
    exponent: 2,
    code: "ZWL",
    locale: EnUs,
    minor_units: 1,
    name: "Zimbabwe Dollar",
    symbol: "Z$",
    symbol_first: true,
};
