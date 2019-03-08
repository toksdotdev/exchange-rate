use crate::currency::Currency;
use crate::exchange::{Exchange, ExchangePair};
use crate::input::PriceUpdate;
use chrono::NaiveDateTime;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Eq, Clone)]
pub struct ExchangePrice {
    timestamp: NaiveDateTime,
    exchange: String,
    source_currency: Currency,
    destination_currency: Currency,
}

impl ExchangePrice {
    pub fn new(
        timestamp: NaiveDateTime,
        exchange: String,
        source_currency: Currency,
        destination_currency: Currency,
    ) -> Self {
        Self {
            timestamp,
            exchange,
            source_currency,
            destination_currency,
        }
    }

    pub fn timestamp(&self) -> &NaiveDateTime {
        &self.timestamp
    }

    pub fn source_currency(&self) -> &Currency {
        &self.source_currency
    }

    pub fn destination_currency(&self) -> &Currency {
        &self.destination_currency
    }
}

impl PartialEq for ExchangePrice {
    fn eq(&self, other: &ExchangePrice) -> bool {
        self.destination_currency == other.destination_currency
            && self.exchange == other.exchange
            && self.source_currency == other.source_currency
    }
}

impl Hash for ExchangePrice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.exchange.hash(state);
        self.destination_currency.hash(state);
        self.source_currency.hash(state);
    }
}

impl From<&ExchangePrice> for ExchangePair {
    fn from(ex: &ExchangePrice) -> (Exchange, Exchange) {
        (
            Exchange::new(to_static_str(&ex.exchange), ex.source_currency),
            Exchange::new(to_static_str(&ex.exchange), ex.destination_currency),
        )
    }
}

impl From<&PriceUpdate> for ExchangePrice {
    fn from(update: &PriceUpdate) -> Self {
        ExchangePrice::new(
            *update.timestamp(),
            update.exchange().to_string(),
            *update.destination_currency(),
            *update.source_currency(),
        )
    }
}

// Look for an improvement for this
fn to_static_str(s: &str) -> &'static str {
    Box::leak(s.to_owned().into_boxed_str())
}
