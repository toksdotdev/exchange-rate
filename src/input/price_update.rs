use crate::currency::{Currency, CurrencyParseError};
use crate::exchange::Exchange;
use crate::exchange_type::ExchangeType;
use crate::exchange_type::ExchangeTypeParseError;
use chrono::{prelude::*, ParseError};
use rust_decimal::{Decimal, Error};
use std::hash::Hash;
use std::hash::Hasher;
use std::str::FromStr;

#[derive(Debug, Eq, Clone)]
pub struct PriceUpdate {
    timestamp: NaiveDateTime,
    exchange: ExchangeType,
    source_currency: Currency,
    destination_currency: Currency,
    src_to_dest_rate: Decimal,
}

#[derive(Debug)]
pub enum PriceUpdateParseError {
    IncompleteData,
    TimestampError(ParseError),
    InvalidFactor,
    InvalidRate(Error),
    InvalidCurrency(CurrencyParseError),
    InvalidExchangeType(ExchangeTypeParseError),
}

impl PriceUpdate {
    pub fn new(
        timestamp: NaiveDateTime,
        exchange: ExchangeType,
        source_currency: Currency,
        destination_currency: Currency,
        forward_factor: Decimal,
        backward_factor: Decimal,
    ) -> Self {
        Self {
            timestamp,
            exchange,
            source_currency,
            destination_currency,
            src_to_dest_rate: forward_factor / backward_factor,
        }
    }

    pub fn timestamp(&self) -> &NaiveDateTime {
        &self.timestamp
    }

    pub fn exchange(&self) -> &ExchangeType {
        &self.exchange
    }

    pub fn source_currency(&self) -> &Currency {
        &self.source_currency
    }

    pub fn destination_currency(&self) -> &Currency {
        &self.destination_currency
    }
    pub fn forward_factor(&self) -> &Decimal {
        &self.src_to_dest_rate
    }
    pub fn backward_factor(&self) -> Decimal {
        let a: Decimal = 1.into();
        a / self.src_to_dest_rate
    }

    /// As an improvement, use serde serialize
    pub fn from_str(data: &str) -> Result<Self, PriceUpdateParseError> {
        let mut values = data.split_whitespace();

        if values.clone().count() != 6 {
            return Err(PriceUpdateParseError::IncompleteData);
        }

        let datetime =
            NaiveDateTime::parse_from_str(values.next().unwrap(), "%Y-%m-%dT%H:%M:%S%z")?;

        Ok(Self::new(
            datetime,
            values.next().unwrap().parse()?,
            values.next().unwrap().parse()?,
            values.next().unwrap().parse()?,
            values.next().unwrap().parse()?,
            values.next().unwrap().parse()?,
        ))
    }
}

impl From<ParseError> for PriceUpdateParseError {
    fn from(error: ParseError) -> Self {
        PriceUpdateParseError::TimestampError(error)
    }
}

impl From<std::num::ParseFloatError> for PriceUpdateParseError {
    fn from(_error: std::num::ParseFloatError) -> Self {
        PriceUpdateParseError::InvalidFactor
    }
}

impl From<CurrencyParseError> for PriceUpdateParseError {
    fn from(error: CurrencyParseError) -> Self {
        PriceUpdateParseError::InvalidCurrency(error)
    }
}

impl From<ExchangeTypeParseError> for PriceUpdateParseError {
    fn from(error: ExchangeTypeParseError) -> Self {
        PriceUpdateParseError::InvalidExchangeType(error)
    }
}

impl From<Error> for PriceUpdateParseError {
    fn from(error: Error) -> Self {
        PriceUpdateParseError::InvalidRate(error)
    }
}

impl PartialEq for PriceUpdate {
    fn eq(&self, other: &PriceUpdate) -> bool {
        self.destination_currency == other.destination_currency
            && self.exchange == other.exchange
            && self.source_currency == other.source_currency
    }
}

impl Hash for PriceUpdate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.exchange.hash(state);
        self.destination_currency.hash(state);
        self.source_currency.hash(state);
    }
}

impl From<&PriceUpdate> for (Exchange, Exchange) {
    fn from(ex: &PriceUpdate) -> (Exchange, Exchange) {
        (
            Exchange::new(ex.exchange, ex.source_currency, ex.timestamp),
            Exchange::new(ex.exchange, ex.destination_currency, ex.timestamp),
        )
    }
}
