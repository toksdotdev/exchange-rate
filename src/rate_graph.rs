use crate::currency::Currency;
use crate::exchange::{Exchange, ExchangePair, ExchangePrice};
use crate::input::PriceUpdate;
use permutate::Permutator;
use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

struct RateGraph {
    prices: HashMap<Currency, HashSet<ExchangePrice>>,
    graph: DiGraphMap<Exchange, f32>,
}

impl RateGraph {
    /// Add a given exchange price to list of pricess. If the price is not the
    /// latest, it becomes discarded.
    fn create_or_update_price(&mut self, price: &ExchangePrice) {
        self.prices
            .entry(*price.source_currency())
            .and_modify(|set| modify_exchange_price(set, price))
            .or_insert_with(|| HashSet::from_iter(vec![price.clone()]));
    }

    pub fn add_prices(&mut self, updates: Vec<PriceUpdate>) {
        for exchange in updates.iter() {
            let exchange_price = ExchangePrice::from(exchange);
            self.create_or_update_price(&exchange_price);
            let (ex1, ex2) = ExchangePair::from(&exchange_price);
            let src_node = self.graph.add_node(ex1);
            let dst_node = self.graph.add_node(ex2);

            self.graph
                .add_edge(src_node, dst_node, *exchange.forward_factor());
            self.graph
                .add_edge(dst_node, src_node, *exchange.backward_factor());
        }
    }

    fn grouped_currency_nodes(&self) {
        self.prices.iter().for_each(|currency| {
            let a: Vec<&ExchangePrice> = currency.1.iter().collect();
            let permutator = Permutator::new(&[a.as_slice()]);

            if let Some(mut permutation) = permutator.next() {
                for element in &permutation {
                    let _ = stdout.write(element.as_bytes());
                }
                let _ = stdout.write(b"\n");
                while permutator.next_with_buffer(&mut permutation) {
                    for element in &permutation {
                        let _ = stdout.write(element.as_bytes());
                    }
                    let _ = stdout.write(b"\n");
                }
            }
        });
    }

    fn get_exchange_for_currency(&self, currency: &Currency) -> Option<&HashSet<ExchangePrice>> {
        self.prices.get(currency)
    }
}

// Only update if the price is far more recent
fn modify_exchange_price(entry: &mut HashSet<ExchangePrice>, price: &ExchangePrice) {
    match entry.get(price) {
        // Todo: As an improvement, create a struct Called OrderedHashSet<T>
        Some(prev_price) if prev_price.timestamp() > price.timestamp() => {
            entry.remove(price);
            entry.insert(price.clone());
        }
        None => {
            entry.insert(price.clone());
        }
        _ => {} // Already exists and is latest.
    };
}
