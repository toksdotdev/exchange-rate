use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum ExchangeType {
    GDAX,
    KRAKEN,
}

#[derive(Debug)]
pub enum ExchangeTypeParseError {
    UnsupportedExchange,
}

impl FromStr for ExchangeType {
    type Err = ExchangeTypeParseError;

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
