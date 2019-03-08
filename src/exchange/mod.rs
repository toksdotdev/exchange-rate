mod price;
pub use price::ExchangePrice;

use crate::currency::Currency;

#[derive(Eq, PartialEq, Hash, PartialOrd, Clone, Copy, Ord)]
pub struct Exchange(&'static str, Currency);

/// ExchangePair comprising of a both (Source, Destination).
pub type ExchangePair = (Exchange, Exchange);

impl Exchange {
    pub fn new(exchange: &'static str, currency: Currency) -> Self {
        Self(exchange, currency)
    }
}
