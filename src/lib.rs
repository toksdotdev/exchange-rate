pub mod exchange;
mod input;
mod utils;

pub mod prelude {
    //! Basic tools to help make detrermine the best exchange trades and exchange rates.
    //!
    //! It involves:
    //! - Determining a sequence of trades and transfers across exchanges
    //! to convert the cryptocurrency to fiat currency with a suitable exchange rate.
    //! - Providing the best possible exchange rate to our customers.
    pub use super::{
        exchange::{Currency, ExchangeType, RateGraph},
        input::{ExchangeRateRequest, PriceUpdate},
        utils::output,
    };
    pub use chrono::NaiveDate;
    pub use rust_decimal::Decimal;
    pub use std::str::FromStr;

    pub mod drawing {
        //! A collection of tools for representing graphs in a more readable format.
        pub use petgraph::dot::{Config, Dot};
    }
}
