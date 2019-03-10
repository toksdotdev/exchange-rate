use crate::currency::{Currency, CurrencyParseError};
use crate::exchange::Exchange;
use crate::exchange_type::ExchangeType;
use crate::exchange_type::ExchangeTypeParseError;
use chrono::NaiveDate;

#[derive(Debug, PartialEq)]
pub struct ExchangeRateRequest {
    source_exchange: ExchangeType,
    source_currency: Currency,
    destination_exchange: ExchangeType,
    destination_currency: Currency,
}

#[derive(Debug)]
pub enum ExchangeRateRequestParseError {
    IncompleteData,
    InvalidCurrency(CurrencyParseError),
    InvalidExchange(ExchangeTypeParseError),
}

impl ExchangeRateRequest {
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

    pub fn from_str(data: &str) -> Result<Self, ExchangeRateRequestParseError> {
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

impl From<CurrencyParseError> for ExchangeRateRequestParseError {
    fn from(error: CurrencyParseError) -> Self {
        ExchangeRateRequestParseError::InvalidCurrency(error)
    }
}
impl From<ExchangeTypeParseError> for ExchangeRateRequestParseError {
    fn from(error: ExchangeTypeParseError) -> Self {
        ExchangeRateRequestParseError::InvalidExchange(error)
    }
}

impl From<&ExchangeRateRequest> for (Exchange, Exchange) {
    fn from(ex: &ExchangeRateRequest) -> (Exchange, Exchange) {
        (
            Exchange::new(
                ex.source_exchange,
                ex.source_currency,
                NaiveDate::from_ymd(2015, 9, 5).and_hms(23, 56, 4),
            ),
            Exchange::new(
                ex.destination_exchange,
                ex.destination_currency,
                NaiveDate::from_ymd(2015, 9, 5).and_hms(23, 56, 4),
            ),
        )
    }
}
