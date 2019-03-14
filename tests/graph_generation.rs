use exchange_rate::exchange::{ExchangeVertex, RateGraphError};
use exchange_rate::prelude::*;
use matches::assert_matches;
use num_traits::cast::FromPrimitive;

fn price_updates() -> Vec<PriceUpdate> {
    vec![
        PriceUpdate::new(
            NaiveDate::from_ymd(2017, 11, 1).and_hms(9, 42, 23),
            ExchangeType::KRAKEN,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1000.0").unwrap(),
            Decimal::from_str("0.0009").unwrap(),
        ),
        PriceUpdate::new(
            NaiveDate::from_ymd(2016, 11, 1).and_hms(1, 23, 23),
            ExchangeType::GDAX,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1001.0").unwrap(),
            Decimal::from_str("0.0008").unwrap(),
        ),
    ]
}

fn expected_nodes() -> Vec<ExchangeVertex> {
    vec![
        ExchangeVertex::new(ExchangeType::GDAX, Currency::BTC),
        ExchangeVertex::new(ExchangeType::GDAX, Currency::USD),
        ExchangeVertex::new(ExchangeType::KRAKEN, Currency::BTC),
        ExchangeVertex::new(ExchangeType::KRAKEN, Currency::USD),
    ]
}

#[test]
fn test_generated_graph() {
    let rate_graph = RateGraph::from(price_updates());
    let graph = rate_graph.graph();
    let expected_nodes = expected_nodes();

    assert_eq!(graph.node_count(), expected_nodes.len());
    assert_eq!(graph.edge_count(), 12);

    for node in expected_nodes.iter() {
        assert!(graph.contains_node(*node));
    }
}

#[test]
fn test_shortest_path_cost_correctness() {
    let rate_graph = RateGraph::from(price_updates());

    let requests = vec![
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

    let expected_rate = vec![0.000_001_8, 1.0];
    for (rqst, rate) in requests.iter().zip(expected_rate) {
        let (_, cost) = rate_graph.full_path(&rqst).unwrap();
        assert_eq!(cost, Decimal::from_f32(rate).unwrap());
    }
}

#[test]
fn test_generated_path_correctness() {
    let rate_graph = RateGraph::from(price_updates());

    let request = vec![
        ExchangeRateRequest::new(
            ExchangeType::KRAKEN,
            Currency::USD,
            ExchangeType::GDAX,
            Currency::LTC,
        ),
        ExchangeRateRequest::new(
            ExchangeType::GDAX,
            Currency::BTC,
            ExchangeType::KRAKEN,
            Currency::USD,
        ),
    ];

    let path_a = vec![
        ExchangeVertex::new(ExchangeType::GDAX, Currency::BTC),
        ExchangeVertex::new(ExchangeType::KRAKEN, Currency::BTC),
        ExchangeVertex::new(ExchangeType::KRAKEN, Currency::USD),
    ];

    let generated_path = rate_graph.full_path(&request[0]);
    assert_matches!(
        generated_path,
        Err(RateGraphError::NoEdgesBetweenNodes(_, _))
    );

    let generated_path = rate_graph.full_path(&request[1]).unwrap();
    for (input, output) in generated_path.0.iter().zip(path_a) {
        assert_eq!(input.currency(), output.currency());
        assert_eq!(input.exchange(), output.exchange());
    }

    assert_eq!(generated_path.1, 2.into());
}
