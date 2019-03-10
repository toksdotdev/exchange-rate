// mod price;
use crate::exchange_type::ExchangeType;
use chrono::NaiveDateTime;
// pub use price::ExchangePrice;

use crate::currency::Currency;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Clone, Copy, Ord)]
// pub struct Exchange<'a>(&'a ExchangeType, &'a Currency, &'a NaiveDateTime);
pub struct Exchange(ExchangeType, Currency, NaiveDateTime);

/// ExchangePair comprising of a both (Source, Destination).
pub type ExchangePair = (Exchange, Exchange);
pub type ExchangePairRef<'a> = (&'a Exchange, &'a Exchange);

impl Exchange {
    pub fn new(exchange: ExchangeType, currency: Currency, datetime: NaiveDateTime) -> Self {
        Self(exchange, currency, datetime)
    }

    pub fn timestamp(&self) -> &NaiveDateTime {
        &self.2
    }

    pub fn currency(&self) -> &Currency {
        &self.1
    }

    pub fn set_timestamp(&mut self, timestamp: NaiveDateTime) {
        self.2 = timestamp;
    }
}
