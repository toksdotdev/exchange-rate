# Exchange Rate Problem

In order to provide our customers a product that lets them spend cryptocurrencies to buy goods from merchants who only
accept fiat currency, we need to solve two problems:

1. Determine a sequence of trades and transfers across exchanges to convert the cryptocurrency to fiat currency with a
   suitable exchange rate
2. Provide the best possible exchange rate to our customers


## Language
- `Rust (Stable V 1.33.0)`

## Usage

All necessary functionalities required to easily get started have been abstracted into the `prelude` modules.

All price updates are fed into the rate graph, and subsequently, exchange rate request can be passed into the `full_path` to get the `(sequence_of_trades, best_exchange_rate)`.

```Rust
use exchange_rate::prelude::*;

fn main() {
    let price_updates = vec![
        PriceUpdate::new(
            NaiveDate::from_ymd(2015, 9, 5).and_hms(23, 56, 4),
            ExchangeType::KRAKEN,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1.0").unwrap(),
            Decimal::from_str("0.0000009").unwrap(),
        ),
        PriceUpdate::new(
            NaiveDate::from_ymd(2019, 2, 19).and_hms(16, 26, 34),
            ExchangeType::GDAX,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1001.0").unwrap(),
            Decimal::from_str("0.0008").unwrap(),
        ),
    ];

    let result = ExchangeRateRequest::new(
        ExchangeType::KRAKEN,
        Currency::USD,
        ExchangeType::GDAX,
        Currency::BTC,
    );

    let rate_graph = RateGraph::from(price_updates);
    let (full_path, cost) = rate_graph.full_path(&rqst).unwrap();
    println!(format!("{}", output(&full_path, cost)));
}
```


## Tests
Tests can be located in the [tests](./tests) folder, and currently has been setup to use [Travis CI](https://travis-ci.org) via [.travis.yml](./.travis.yml).