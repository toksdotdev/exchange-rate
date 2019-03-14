use crate::exchange::{
    Currency, CurrencyParseError, ExchangeType, ExchangeTypeParseError, ExchangeVertex,
    ExchangeVertexPair,
};
use std::str::FromStr;

/// Represents a request to convert from given currency on
/// a source exchange to another currency on a destionation exchange.
#[derive(Debug, PartialEq)]
pub struct ExchangeRateRequest {
    source_exchange: ExchangeType,
    source_currency: Currency,
    destination_exchange: ExchangeType,
    destination_currency: Currency,
}

/// Error that occur while parsing into ExchangeRateRequest
/// from a given string slice(`&str`).
#[derive(Debug)]
pub enum ExchangeRateRequestParseError {
    IncompleteData,
    InvalidCurrency(CurrencyParseError),
    InvalidExchange(ExchangeTypeParseError),
}

impl ExchangeRateRequest {
    // Create a new ExchangeRateRequest from specified values.
    ///
    /// Example:
    ///
    /// ```
    /// use exchange_rate::prelude::{ExchangeRateRequest, ExchangeType, Currency};
    ///
    /// let request = ExchangeRateRequest::new(
    ///     ExchangeType::KRAKEN,
    ///     Currency::USD,
    ///     ExchangeType::GDAX,
    ///     Currency::LTC,
    /// );
    ///
    /// ```
    pub fn new(
        source_exchange: ExchangeType,
        source_currency: Currency,
        destination_exchange: ExchangeType,
        destination_currency: Currency,
    ) -> Self {
        Self {
            source_currency,
            source_exchange,
            destination_currency,
            destination_exchange,
        }
    }
}

impl From<CurrencyParseError> for ExchangeRateRequestParseError {
    /// Convert from `CurrencyParseError` to `ExchangeRateRequestParseError`.
    fn from(error: CurrencyParseError) -> Self {
        ExchangeRateRequestParseError::InvalidCurrency(error)
    }
}

impl From<ExchangeTypeParseError> for ExchangeRateRequestParseError {
    /// Convert from `ExchangeTypeParseError` to `ExchangeRateRequestParseError`.
    fn from(error: ExchangeTypeParseError) -> Self {
        ExchangeRateRequestParseError::InvalidExchange(error)
    }
}

impl From<&ExchangeRateRequest> for ExchangeVertexPair {
    /// Convert from  reference of `ExchangeRateRequest` to `ExchangeVertexPair`.
    fn from(ex: &ExchangeRateRequest) -> ExchangeVertexPair {
        (
            ExchangeVertex::new(ex.source_exchange, ex.source_currency),
            ExchangeVertex::new(ex.destination_exchange, ex.destination_currency),
        )
    }
}

impl FromStr for ExchangeRateRequest {
    type Err = ExchangeRateRequestParseError;

    /// Attempts to create an ExchangeRateRequest from a given `&str`.
    /// If an error occurs, during the conversion, `ExchangeRateRequestParseError`.
    ///
    /// The string slice must  follows the format:
    /// `EXCHANGE_RATE_REQUEST <source_exchange> <source_currency> <destination_exchange> <destination_currency>`
    ///
    /// Example:
    ///
    ///
    /// use exchange_rate::prelude::ExchangeRateRequest;
    ///
    /// let sliced_request = "EXCHANGE_RATE_REQUEST KRAKEN USD GDAX LTC";
    /// let output = ExchangeRateRequest::new(
    ///     ExchangeType::KRAKEN,
    ///     Currency::USD,
    ///     ExchangeType::GDAX,
    ///     Currency::LTC,
    /// );
    ///
    /// assert_eq!(ExchangeRateRequest::from_str(sliced_request), output);
    ///
    ///
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut values = data.split_whitespace();

        if values.clone().count() != 5 {
            return Err(ExchangeRateRequestParseError::IncompleteData);
        }

        values.next(); // Skips `EXCHANGE_RATE_REQUEST`

        Ok(Self {
            source_exchange: values.next().unwrap().parse()?,
            source_currency: values.next().unwrap().parse()?,
            destination_exchange: values.next().unwrap().parse()?,
            destination_currency: values.next().unwrap().parse()?,
        })
    }
}
