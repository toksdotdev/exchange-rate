use crate::currency::{Currency, CurrencyParseError};

#[derive(Debug, PartialEq)]
pub struct ExchangeRateRequest {
    source_exchange: String,
    source_currency: Currency,
    destination_exchange: String,
    destination_currency: Currency,
}

#[derive(Debug)]
pub enum ExchangeRateRequestParseError {
    IncompleteData,
    InvalidCurrency(CurrencyParseError),
}

impl ExchangeRateRequest {
    pub fn new(
        source_exchange: String,
        source_currency: Currency,
        destination_exchange: String,
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
            source_exchange: values.next().unwrap().to_string(),
            source_currency: values.next().unwrap().parse()?,
            destination_exchange: values.next().unwrap().to_string(),
            destination_currency: values.next().unwrap().parse()?,
        })
    }
}

impl From<CurrencyParseError> for ExchangeRateRequestParseError {
    fn from(error: CurrencyParseError) -> Self {
        ExchangeRateRequestParseError::InvalidCurrency(error)
    }
}
