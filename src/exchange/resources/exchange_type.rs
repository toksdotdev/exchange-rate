use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// An enum of all suporeted exchange type available for use in an exchange.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum ExchangeType {
    GDAX,
    KRAKEN,
}

/// Errors that occur while parsing a given string value into a valid EchangeType.
#[derive(Debug)]
pub enum ExchangeTypeParseError {
    UnsupportedExchange,
}

impl FromStr for ExchangeType {
    type Err = ExchangeTypeParseError;

    /// Get an equivalent ExchangeType type from a given string slice.
    fn from_str(s: &str) -> Result<ExchangeType, Self::Err> {
        match s.to_lowercase().as_ref() {
            "gdax" => Ok(ExchangeType::GDAX),
            "kraken" => Ok(ExchangeType::KRAKEN),
            _ => Err(ExchangeTypeParseError::UnsupportedExchange),
        }
    }
}

impl Display for ExchangeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let currency = match self {
            ExchangeType::GDAX => "GDAX",
            ExchangeType::KRAKEN => "KRAKEN",
        };

        write!(f, "{}", currency)
    }
}
