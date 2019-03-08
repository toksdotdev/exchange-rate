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
mod input;
// mod rate_graph;
mod tests;

mod prelude {
    pub use super::currency::Currency;
    pub use super::input::ExchangeRateRequest;
    pub use super::input::PriceUpdate;
}

fn main() {
    use itertools;

    let mut data = [1, 2, 3, 4, 5, 6];
    data.iter().combination(2);
}
