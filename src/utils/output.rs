use crate::exchange::ExchangeVertex;
use rust_decimal::Decimal;

pub fn output(full_path: &[ExchangeVertex], cost: Decimal) -> String {
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
