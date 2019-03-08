use crate::currency::{Currency, CurrencyParseError};
use chrono::{prelude::*, ParseError};

#[derive(Debug, PartialEq)]
pub struct PriceUpdate {
    timestamp: NaiveDateTime,
    exchange: String,
    source_currency: Currency,
    destination_currency: Currency,
    forward_factor: f32,
    backward_factor: f32,
}

#[derive(Debug)]
pub enum PriceUpdateParseError {
    IncompleteData,
    TimestampError(ParseError),
    InvalidFactor,
    InvalidCurrency(CurrencyParseError),
}

impl PriceUpdate {
    pub fn new(
        timestamp: NaiveDateTime,
        exchange: &str,
        source_currency: Currency,
        destination_currency: Currency,
        forward_factor: f32,
        backward_factor: f32,
    ) -> Self {
        Self {
            timestamp,
            exchange: exchange.to_string(),
            source_currency,
            destination_currency,
            forward_factor,
            backward_factor,
        }
    }

    pub fn timestamp(&self) -> &NaiveDateTime {
        &self.timestamp
    }

    pub fn exchange(&self) -> &str {
        &self.exchange
    }

    pub fn source_currency(&self) -> &Currency {
        &self.source_currency
    }

    pub fn destination_currency(&self) -> &Currency {
        &self.destination_currency
    }
    pub fn forward_factor(&self) -> &f32 {
        &self.forward_factor
    }

    pub fn backward_factor(&self) -> &f32 {
        &self.backward_factor
    }

    /// As an improvement, use serde serialize
    pub fn from_str(data: &str) -> Result<Self, PriceUpdateParseError> {
        let mut values = data.split_whitespace();

        if values.clone().count() != 6 {
            return Err(PriceUpdateParseError::IncompleteData);
        }

        Ok(Self {
            timestamp: values.next().unwrap().parse()?,
            exchange: values.next().unwrap().to_string(),
            source_currency: values.next().unwrap().parse()?,
            destination_currency: values.next().unwrap().parse()?,
            forward_factor: values.next().unwrap().parse()?,
            backward_factor: values.next().unwrap().parse()?,
        })
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

// impl Into<ExchangePrice> for PriceUpdate {
//     fn into(self) -> ExchangePrice {
//         ExchangePrice::new(
//             self.timestamp,
//             self.exchange,
//             self.destination_currency,
//             self.source_currency,
//         )
//     }
// }
