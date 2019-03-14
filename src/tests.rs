use super::prelude::{Currency, ExchangeRateRequest, ExchangeType, PriceUpdate, RateGraph};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn parse_price_update() {
    let price_updates = r#"2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009
    2017-11-01T09:43:23+00:00 GDAX BTC USD 1001.0 0.0008"#;

    let result = vec![
        PriceUpdate::new(
            "2017-11-01T09:42:23+00:00".parse().unwrap(),
            ExchangeType::KRAKEN,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1000.0").unwrap(),
            Decimal::from_str("0.0009").unwrap(),
        ),
        PriceUpdate::new(
            "2017-11-01T09:43:23+00:00".parse().unwrap(),
            ExchangeType::GDAX,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1001.0").unwrap(),
            Decimal::from_str("0.0008").unwrap(),
        ),
    ];

    for (input, output) in price_updates.lines().zip(result) {
        assert_eq!(PriceUpdate::from_str(input).unwrap(), output);
    }
}

#[test]
fn parse_exchange_rate() {
    let rate_requests = r#"EXCHANGE_RATE_REQUEST KRAKEN USD GDAX LTC
    EXCHANGE_RATE_REQUEST GDAX BTC GDAX LTC"#;

    let result = vec![
        ExchangeRateRequest::new(
            ExchangeType::KRAKEN,
            Currency::USD,
            ExchangeType::GDAX,
            Currency::LTC,
        ),
        ExchangeRateRequest::new(
            ExchangeType::GDAX,
            Currency::BTC,
            ExchangeType::GDAX,
            Currency::LTC,
        ),
    ];

    for (input, output) in rate_requests.lines().zip(result) {
        assert_eq!(ExchangeRateRequest::from_str(input).unwrap(), output);
    }
}

#[test]
fn test_generated_graph() {
    let price_updates = vec![
        PriceUpdate::new(
            NaiveDate::from_ymd(2017, 11, 1).and_hms(9, 42, 23),
            ExchangeType::KRAKEN,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1000.0").unwrap(),
            Decimal::from_str("0.0009").unwrap(),
        ),
        PriceUpdate::new(
            "2017-11-01T09:43:23+00:00".parse().unwrap(),
            ExchangeType::GDAX,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1001.0").unwrap(),
            Decimal::from_str("0.0008").unwrap(),
        ),
    ];

    let _ = RateGraph::from(price_updates);
    // dbg!(graph);
}

#[test]
fn test_shortest_path_correctness() {
    unimplemented!();
}

#[test]
fn test_output() {
    unimplemented!();
}
