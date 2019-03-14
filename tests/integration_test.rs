use exchange_rate::exchange::ExchangeVertex;
use exchange_rate::prelude::*;
use std::fs::read;

/// Some random dummy price updates.
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

#[test]
fn test_output() {
    let file = &read("tests/inputs/easy/integration_exchange_rate_request.txt").unwrap();
    let rate_requests = String::from_utf8_lossy(file);
    let file = &read("tests/outputs/easy/output.txt").unwrap();
    let output_string = String::from_utf8_lossy(file);

    let rate_graph = RateGraph::from(price_updates());

    let mut out = "".to_string();
    for (index, rqst) in rate_requests.lines().enumerate() {
        let request = ExchangeRateRequest::from_str(rqst).unwrap();
        let (full_path, cost) = rate_graph.full_path(&request).unwrap();

        if index == rate_requests.lines().count() - 1 {
            out = format!("{}{}", out, output(&full_path, cost));
        } else if index == 0 {
            out = format!("{}{}\n", out, output(&full_path, cost));
        } else {
            out = format!("{}\n{}", out, output(&full_path, cost));
        }
    }

    assert_eq!(output_string, out);
}

fn output(full_path: &[ExchangeVertex], cost: Decimal) -> String {
    let mut full_path = full_path.iter();
    let src = full_path.next().unwrap();
    let dst = full_path.clone().last().unwrap();

    let mut out = format!(
        "BEST_RATES_BEGIN {} {} {} {} {}\n<{}, {}>\n",
        src.exchange(),
        src.currency(),
        dst.exchange(),
        dst.currency(),
        cost,
        src.exchange(),
        src.currency(),
    );

    for node in full_path {
        out.push_str(&format!("<{}, {}>\n", node.exchange(), node.currency()));
    }

    out.push_str("BEST_RATES_END");
    out
}
