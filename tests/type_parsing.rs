use exchange_rate::prelude::*;
use std::fs::read;

#[test]
fn parse_price_update() {
    let file = &read("tests/inputs/easy/price_updates.txt").unwrap();
    let price_updates = String::from_utf8_lossy(file);

    let result = vec![
        PriceUpdate::new(
            NaiveDate::from_ymd(2017, 11, 1).and_hms(9, 42, 23),
            ExchangeType::KRAKEN,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1000.0").unwrap(),
            Decimal::from_str("0.0009").unwrap(),
        ),
        PriceUpdate::new(
            NaiveDate::from_ymd(2017, 11, 1).and_hms(9, 43, 23),
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
fn parse_exchange_rate_request() {
    let file = &read("tests/inputs/easy/exchange_rate_requests.txt").unwrap();
    let rate_requests = String::from_utf8_lossy(file);

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
        ExchangeRateRequest::new(
            ExchangeType::GDAX,
            Currency::USD,
            ExchangeType::KRAKEN,
            Currency::BTC,
        ),
        ExchangeRateRequest::new(
            ExchangeType::GDAX,
            Currency::BTC,
            ExchangeType::GDAX,
            Currency::USD,
        ),
    ];

    for (input, output) in rate_requests.lines().zip(result) {
        assert_eq!(ExchangeRateRequest::from_str(input).unwrap(), output);
    }
}
