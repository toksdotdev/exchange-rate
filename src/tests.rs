use crate::prelude::{Currency, ExchangeRateRequest, PriceUpdate};

#[test]
fn parse_price_update() {
    let price_updates = r#"2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009
    2017-11-01T09:43:23+00:00 GDAX BTC USD 1001.0 0.0008"#;

    let result = vec![
        PriceUpdate::new(
            "2017-11-01T09:42:23+00:00".parse().unwrap(),
            "KRAKEN",
            Currency::BTC,
            Currency::USD,
            1000.0,
            0.0009,
        ),
        PriceUpdate::new(
            "2017-11-01T09:43:23+00:00".parse().unwrap(),
            "GDAX",
            Currency::BTC,
            Currency::USD,
            1001.0,
            0.0008,
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
            "KRAKEN".to_string(),
            Currency::USD,
            "GDAX".to_string(),
            Currency::LTC,
        ),
        ExchangeRateRequest::new(
            "GDAX".to_string(),
            Currency::BTC,
            "GDAX".to_string(),
            Currency::LTC,
        ),
    ];

    for (input, output) in rate_requests.lines().zip(result) {
        assert_eq!(ExchangeRateRequest::from_str(input).unwrap(), output);
    }
}

#[test]
fn test_generated_graph() {
    unimplemented!();
}

#[test]
fn test_shortest_path_correctness() {
    unimplemented!();
}

#[test]
fn test_output() {
    unimplemented!();
}
