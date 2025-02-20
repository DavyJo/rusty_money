use crate::Locale;

#[cfg(feature = "crypto")]
mod crypto_currencies;
#[cfg(feature = "crypto")]
pub use crypto_currencies::crypto;

#[cfg(feature = "iso")]
mod iso_currencies;
#[cfg(feature = "iso")]
pub use iso_currencies::iso;

/// Pre-requisite for a Currency to be accepted by a Money.
pub trait FormattableCurrency: PartialEq + Eq + Copy + ToString + Ord {
    fn exponent(&self) -> u32;

    fn code(&self) -> &'static str;

    fn locale(&self) -> Locale;

    fn symbol(&self) -> &'static str;

    fn symbol_first(&self) -> bool;
}