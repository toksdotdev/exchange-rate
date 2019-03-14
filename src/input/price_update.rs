use crate::exchange::{
    Currency, CurrencyParseError, ExchangeType, ExchangeTypeParseError, ExchangeVertex,
    ExchangeVertexPair,
};
use chrono::{prelude::NaiveDateTime, ParseError};
use rust_decimal::{Decimal, Error};
use std::str::FromStr;

/// A representation of a price update.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PriceUpdate {
    timestamp: NaiveDateTime,
    exchange: ExchangeType,
    source_currency: Currency,
    destination_currency: Currency,
    backward_factor: Decimal,
}

/// Errors that can occur while parsing a string slice(`&str`) into a given `PriceUpdate`.
#[derive(Debug)]
pub enum PriceUpdateParseError {
    InvalidFactor,
    IncompleteData,
    InvalidRate(Error),
    TimestampError(ParseError),
    InvalidCurrency(CurrencyParseError),
    InvalidExchangeType(ExchangeTypeParseError),
}

impl PriceUpdate {
    /// Create a new `PriceUpdate` from the specified values.
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
            backward_factor: backward_factor / forward_factor,
        }
    }

    /// Get the timestamp of a price update.
    pub fn timestamp(&self) -> &NaiveDateTime {
        &self.timestamp
    }

    // Get the exchange type of a price update.
    pub fn exchange(&self) -> &ExchangeType {
        &self.exchange
    }

    /// Get the source currency of a price update.
    pub fn source_currency(&self) -> &Currency {
        &self.source_currency
    }

    /// Get the destination currency of a price update.
    pub fn destination_currency(&self) -> &Currency {
        &self.destination_currency
    }

    /// Get the forward factor of a price update.
    pub fn forward_factor(&self) -> Decimal {
        1.into()
    }

    /// Get the backward factor of a price update.
    pub fn backward_factor(&self) -> &Decimal {
        &self.backward_factor
    }
}

impl FromStr for PriceUpdate {
    type Err = PriceUpdateParseError;

    /// Attempts to create an PriceUpdate from a given `&str`.
    /// If an error occurs, during the conversion, `PriceUpdateParseError`.
    ///
    /// The string slice must  follows the format:
    /// `<timestamp> <exchange> <source_currency> <destination_currency> <forward_factor> <backward_factor>`
    ///
    /// For example: `2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009`
    ///
    /// Example:
    ///
    /// ```
    /// use exchange_rate::prelude::*;
    ///
    /// let sliced_update = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009";
    /// let output = PriceUpdate::new(
    ///     NaiveDate::from_ymd(2015, 9, 5).and_hms(23, 56, 4),
    ///     ExchangeType::KRAKEN,
    ///     Currency::BTC,
    ///     Currency::USD,
    ///     Decimal::from_str("1.0").unwrap(),
    ///     Decimal::from_str("0.0000009").unwrap(),
    /// );
    ///
    /// assert_eq!(PriceUpdate::from_str(sliced_update), output);
    ///
    /// ```
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        // Todo: As an improvement, use `serde` for serialization.
        let mut values = data.split_whitespace();

        if values.clone().count() != 6 {
            return Err(PriceUpdateParseError::IncompleteData);
        }

        Ok(Self::new(
            NaiveDateTime::parse_from_str(values.next().unwrap(), "%Y-%m-%dT%H:%M:%S%z")?,
            values.next().unwrap().parse()?,
            values.next().unwrap().parse()?,
            values.next().unwrap().parse()?,
            values.next().unwrap().parse()?,
            values.next().unwrap().parse()?,
        ))
    }
}

impl From<ParseError> for PriceUpdateParseError {
    /// Convert from `ParseError` to `PriceUpdateParseError`.
    fn from(error: ParseError) -> Self {
        PriceUpdateParseError::TimestampError(error)
    }
}

impl From<std::num::ParseFloatError> for PriceUpdateParseError {
    /// Convert from `std::num::ParseFloatError` to `PriceUpdateParseError`.
    fn from(_: std::num::ParseFloatError) -> Self {
        PriceUpdateParseError::InvalidFactor
    }
}

impl From<CurrencyParseError> for PriceUpdateParseError {
    /// Convert from `CurrencyParseError` to `PriceUpdateParseError`.
    fn from(error: CurrencyParseError) -> Self {
        PriceUpdateParseError::InvalidCurrency(error)
    }
}

impl From<ExchangeTypeParseError> for PriceUpdateParseError {
    /// Convert from `ExchangeTypeParseError` to `PriceUpdateParseError`.
    fn from(error: ExchangeTypeParseError) -> Self {
        PriceUpdateParseError::InvalidExchangeType(error)
    }
}

impl From<rust_decimal::Error> for PriceUpdateParseError {
    /// Convert from `rust_decimal::Error` to `PriceUpdateParseError`.
    fn from(error: rust_decimal::Error) -> Self {
        PriceUpdateParseError::InvalidRate(error)
    }
}

impl From<&PriceUpdate> for ExchangeVertexPair {
    /// Convert from `CurrencyParseError` to `ExchangeRateRequestParseError`.
    fn from(ex: &PriceUpdate) -> ExchangeVertexPair {
        (
            ExchangeVertex::new(ex.exchange, ex.source_currency),
            ExchangeVertex::new(ex.exchange, ex.destination_currency),
        )
    }
}
