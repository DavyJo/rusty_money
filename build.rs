use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

// Struct matching data/currencies.json
#[derive(Deserialize, Debug)]
struct CurrencyDef {
    code: String, // Changed from alphabetic_code
    exponent: u32, // Changed from Option<u32> and decimal_digits
    locale: String, // Keep track of the default locale code string
    minor_units: u64,
    name: String,
    symbol: String,
    symbol_first: bool,
}

// Struct matching data/locales.json
#[derive(Deserialize, Debug)]
struct LocaleDef {
    digit_separator: String, // Keep as String for now, convert if Locale needs char
    exponent_separator: String, // Keep as String for now, convert if Locale needs char
    grouping: u8,
    infinity_symbol: String,
    minus_sign_symbol: String, // Keep as String for now, convert if Locale needs char
    nan_symbol: String,
    separator_pattern: Vec<u8>,
    symbol_first: bool,
    symbol_spacing: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=data/currencies.json");
    println!("cargo:rerun-if-changed=data/locales.json");
    println!("cargo:rerun-if-changed=build.rs");

    // --- Load Currencies ---
    let currencies_path = Path::new("data/currencies.json");
    let currencies_file = File::open(currencies_path)?;
    let currencies_reader = BufReader::new(currencies_file);
    // Parse as Vec<CurrencyDef> since the JSON is an array
    let currencies: Vec<CurrencyDef> = serde_json::from_reader(currencies_reader)?;

    // --- Load Locales ---
    let locales_path = Path::new("data/locales.json");
    let locales_file = File::open(locales_path)?;
    let locales_reader = BufReader::new(locales_file);
    let locales: HashMap<String, LocaleDef> = serde_json::from_reader(locales_reader)?;

    // --- Generate Code ---
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("generated_maps.rs");
    let mut f = File::create(dest_path)?;

    // Header
    writeln!(f, "// This file is generated by build.rs. Do not edit manually.")?;
    writeln!(f, "use crate::currency::Currency;")?;
    writeln!(f, "use crate::locale::Locale;")?;
    writeln!(f, "use once_cell::sync::Lazy;")?;
    writeln!(f, "use rust_decimal::Decimal;")?; // Assuming Currency uses Decimal
    writeln!(f, "use std::collections::HashMap;")?;
    writeln!(f)?;

    // Currency Map Generation
    writeln!(f, "pub static CURRENCIES: Lazy<HashMap<&'static str, Currency>> = Lazy::new(|| {{")?;
    writeln!(f, "    let mut map = HashMap::new();")?;
    // Iterate over the Vec<CurrencyDef>
    for def in currencies {
        // Generate Currency struct literal directly
        // Note: Locale is omitted here. Formatting logic will need to look it up.
        writeln!(
            f,
            r#"    map.insert("{code}", Currency {{
        code: "{code}",
        exponent: {exponent},
        // locale: field removed from Currency struct
        minor_units: {minor_units},
        name: "{name}",
        symbol: "{symbol}",
        symbol_first: {symbol_first},
    }});"#,
            code = def.code,
            exponent = def.exponent,
            minor_units = def.minor_units,
            name = def.name.replace('\\', "\\\\").replace('"', "\\\""), // Escape name
            symbol = def.symbol.replace('\\', "\\\\").replace('\'', "\\'").replace('"', "\\\""), // Escape symbol
            symbol_first = def.symbol_first,
            // locale_code = def.locale // Store locale code if needed later
        )?;
    }
    writeln!(f, "    map")?;
    writeln!(f, "}});")?;
    writeln!(f)?;

    // Locale Map Generation
    writeln!(f, "pub static LOCALES: Lazy<HashMap<&'static str, Locale>> = Lazy::new(|| {{")?;
    writeln!(f, "    let mut map = HashMap::new();")?;
    for (code, def) in locales {
        // Note: Adjust Locale constructor/fields based on its actual definition
        // Potential conversion from String to char for separators might be needed here
        // depending on the Locale struct definition. Assuming Locale::new takes strings for now.
        // Generate Locale struct literal, ensuring String fields are created correctly
        writeln!(
            f,
            r#"    map.insert("{code}", Locale {{
        digit_separator: String::from("{digit_sep}"),
        exponent_separator: String::from("{exp_sep}"),
        grouping: {grouping},
        infinity_symbol: String::from("{inf}"),
        minus_sign_symbol: String::from("{minus}"),
        nan_symbol: String::from("{nan}"),
        separator_pattern: vec![{separator_pattern}],
        symbol_first: {symbol_first},
        symbol_spacing: {symbol_spacing},
    }});"#,
            code = code,
            // Escape strings for Rust code generation
            digit_sep = def.digit_separator.replace('\\', "\\\\").replace('"', "\\\""),
            exp_sep = def.exponent_separator.replace('\\', "\\\\").replace('"', "\\\""),
            grouping = def.grouping,
            inf = def.infinity_symbol.replace('\\', "\\\\").replace('"', "\\\""),
            minus = def.minus_sign_symbol.replace('\\', "\\\\").replace('"', "\\\""),
            nan = def.nan_symbol.replace('\\', "\\\\").replace('"', "\\\""),
            // Directly join numbers for vec![...]
            separator_pattern = def.separator_pattern.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(", "),
            symbol_first = def.symbol_first,
            symbol_spacing = def.symbol_spacing
        )?;
    }
    writeln!(f, "    map")?;
    writeln!(f, "}});")?;

    Ok(())
}
