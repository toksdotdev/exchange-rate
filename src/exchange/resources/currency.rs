use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// An enum of all supported currency values for use in an exchange.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum Currency {
    USD,
    BTC,
    LTC,
}

/// Errors that occur while parsing a given string value into a valid Currency.
#[derive(Debug)]
pub enum CurrencyParseError {
    UnsupportedCurrency,
}

impl FromStr for Currency {
    type Err = CurrencyParseError;

    /// Get an equivalent Currency type from a given string slice.
    fn from_str(s: &str) -> Result<Currency, Self::Err> {
        match s.to_lowercase().as_ref() {
            "usd" => Ok(Currency::USD),
            "btc" => Ok(Currency::BTC),
            "ltc" => Ok(Currency::LTC),
            _ => Err(CurrencyParseError::UnsupportedCurrency),
        }
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let currency = match self {
            Currency::BTC => "BTC",
            Currency::LTC => "LTC",
            Currency::USD => "USD",
        };

        write!(f, "{}", currency)
    }
}
