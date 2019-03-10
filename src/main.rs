//! Goals:
//! - parse input:  price updates
//! - parse input: exchange rate requests
//! - generate the graph
//! - predict the shortest path
//! - display the output
//!
//!  TEST CASES
//!  - check for different unsupported currency

mod currency;
mod exchange;
mod exchange_type;
mod input;
mod rate_graph;
mod tests;
use rust_decimal::Decimal;
use std::str::FromStr;

mod prelude {
    pub use super::currency::Currency;
    pub use super::exchange_type::ExchangeType;
    pub use super::input::ExchangeRateRequest;
    pub use super::input::PriceUpdate;
    pub use super::rate_graph::RateGraph;
}

use chrono::NaiveDate;
fn main() {
    use self::prelude::*;
    // use itertools;

    // let mut data = [1, 2, 3, 4, 5, 6];
    // data.iter().combination(2);

    // use num_rational::Ratio; // 0.2.1
    // let s1: f32 = 2.34244;
    // let s2: f32 = 344.3;
    // let ratio1 = Ratio::from_float(s1).unwrap();
    // let ratio2 = Ratio::from_float(s2).unwrap();
    // println!("{}", (ratio1 * ratio2));

    let price_updates = vec![
        PriceUpdate::new(
            NaiveDate::from_ymd(2015, 9, 5).and_hms(23, 56, 4),
            ExchangeType::KRAKEN,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1000.0").unwrap(),
            Decimal::from_str("0.0009").unwrap(),
        ),
        PriceUpdate::new(
            NaiveDate::from_ymd(2015, 9, 5).and_hms(23, 56, 4),
            ExchangeType::GDAX,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1001.0").unwrap(),
            Decimal::from_str("0.0008").unwrap(),
        ),
    ];

    let result = vec![
        ExchangeRateRequest::new(
            ExchangeType::KRAKEN,
            Currency::USD,
            ExchangeType::GDAX,
            Currency::BTC,
        ),
        ExchangeRateRequest::new(
            ExchangeType::GDAX,
            Currency::BTC,
            ExchangeType::GDAX,
            Currency::USD,
        ),
    ];

    use petgraph::dot::{Config, Dot};
    let graph = RateGraph::from(price_updates);
    dbg!(&graph.path(&result[0]));
    // dbg!(&graph.best_rates_dijkstra(&result[0]));
    // dbg!(&graph);
    // dbg!(Dot::with_config(graph.graph(), &[Config::EdgeNoLabel]));
}
