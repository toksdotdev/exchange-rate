use crate::input::ExchangeRateRequest;
use crate::{
    currency::Currency,
    exchange::{Exchange, ExchangePair},
    input::PriceUpdate,
};
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graphmap::DiGraphMap;
use rust_decimal::Decimal;
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

#[derive(Debug)]
pub enum Error {
    UnknownExchange(Exchange),
    NoEdgesBetweenNodes(Exchange, Exchange),
}

type RatePath = HashSet<Exchange>;

#[derive(Debug)]
pub struct RateGraph {
    exchange_prices: HashMap<Currency, HashSet<Exchange>>,
    graph: DiGraphMap<Exchange, Decimal>,
    rate: DiGraphMap<Exchange, Decimal>,
    next: DiGraphMap<Exchange, Exchange>,
}

impl RateGraph {
    pub fn graph(&self) -> &DiGraphMap<Exchange, Decimal> {
        &self.graph
    }

    // Todo: fix this (dont return  a vector, return impl Iterator<Item=ExchangePair>)
    fn currency_nodes(&self) -> Vec<ExchangePair> {
        self.exchange_prices
            .iter()
            // Todo: Improve on the combination generation.
            .flat_map(|currency| currency.1.iter().combinations(2).map(|e| (*e[0], *e[1])))
            .collect()
    }

    fn add_edge(
        &mut self,
        src: Exchange,
        dst: Exchange,
        forward_factor: Decimal,
        backward_factor: Decimal,
    ) {
        self.graph.add_edge(src, dst, forward_factor);
        self.graph.add_edge(dst, src, backward_factor);
        self.graph.add_edge(src, src, 0.into());
        self.graph.add_edge(dst, dst, 0.into());
    }

    pub fn insert(&mut self, updates: Vec<PriceUpdate>) {
        for update in updates {
            let (ex1, ex2) = self.insert_exchange(&update);
            let src_node = self.graph.add_node(ex1);
            let dst_node = self.graph.add_node(ex2);

            // Todo: Validate later if forward_factor and backward_factor should return a
            // reference to BigRational, instead of cloning.
            self.add_edge(
                src_node,
                dst_node,
                *update.forward_factor(),
                update.backward_factor(),
            );
        }

        self.currency_nodes().iter().for_each(|pair| {
            self.add_edge(pair.0, pair.1, 1.into(), 1.into());
        });

        self.best_rates_floyd();
    }

    /// Add a given exchange price to list of pricess. If the price is not the
    /// latest, it becomes discarded. If it doesnt exist, it inserts it into
    /// the available exchange prices.
    fn insert_exchange(&mut self, price: &PriceUpdate) -> ExchangePair {
        //Todo:  convert tuple to iterator, and refactor this method.
        let (mut ex1, mut ex2) = ExchangePair::from(price);

        self.exchange_prices
            .entry(*ex1.currency())
            .and_modify(|set| modify_exchange_price(set, &mut ex1))
            .or_insert_with(|| HashSet::from_iter(vec![ex1]));

        self.exchange_prices
            .entry(*ex2.currency())
            .and_modify(|set| modify_exchange_price(set, &mut ex2))
            .or_insert_with(|| HashSet::from_iter(vec![ex2]));

        (ex1, ex2)
    }

    fn best_rates_floyd(&mut self) {
        use petgraph::visit::IntoEdgeReferences;

        for (u, v, _) in self.graph.edge_references() {
            let u_v_weight = *self.graph.edge_weight(u, v).unwrap();
            self.rate.add_edge(u, v, u_v_weight);
            self.next.add_edge(u, v, v);
        }

        let rate = self.rate.clone();
        let mut next = self.next.clone();
        let graph1 = self.graph.clone();
        let graph2 = self.graph.clone();
        let graph3 = self.graph.clone();

        for k in graph1.nodes() {
            for i in graph2.nodes() {
                for j in graph3.nodes() {
                    let i_k = rate.edge_weight(i, k);
                    let i_j = rate.edge_weight(i, j);
                    let k_j = rate.edge_weight(k, j);
                    if i_k == None || k_j == None {
                        continue;
                    }

                    if let Some(i_j_weight) = i_j {
                        let total = i_k.unwrap() * k_j.unwrap();
                        if *i_j_weight < total {
                            self.rate.add_edge(i, j, total);
                            next.add_edge(i, j, *next.edge_weight(i, k).unwrap());
                        }
                    }
                }
            }
        }

        use petgraph::dot::{Config, Dot};
        dbg!(Dot::with_config(&rate, &[]));
        for (u, v, _) in self.rate.edge_references() {
            let u_v_weight = self.rate.edge_weight(u, v).unwrap();
            // dbg!(u_v_weight);
        }
    }

    pub fn best_rates_dijkstra(&self, request: &ExchangeRateRequest) -> Result<(), Error> {
        use petgraph::visit::EdgeRef;
        let (ex1, ex2): ExchangePair = ExchangePair::from(request);
        if !self.graph.contains_node(ex1) {
            return Err(Error::UnknownExchange(ex1));
        }

        if !self.graph.contains_node(ex2) {
            return Err(Error::UnknownExchange(ex2));
        }

        let _scores = dijkstra(
            self.graph(),
            self.graph.nodes().find(|s| *s == ex1).unwrap(),
            self.graph.nodes().find(|s| *s == ex2),
            |e| decimal_to_f64(*e.weight()),
        );

        // dbg!(scores);

        Ok(())
    }

    pub fn path(&self, request: &ExchangeRateRequest) -> Result<RatePath, Error> {
        let (u, v): ExchangePair = ExchangePair::from(request);
        dbg!(u);
        dbg!(v);

        if !self.next.contains_edge(u, v) {
            return Err(Error::NoEdgesBetweenNodes(u, v));
        }

        let mut u = u;
        let mut path = RatePath::default();
        while u != v {
            u = *self.next.edge_weight(u, v).unwrap();
            path.insert(u);
        }

        Ok(path)
    }
}

// Modifies the exchange if it alreasy exists if the price is far more recent
fn modify_exchange_price(entry: &mut HashSet<Exchange>, new_price: &mut Exchange) {
    match entry.get(new_price) {
        None => {
            entry.insert(*new_price);
        }

        // Replace old price with new price (with the goal of updating the timeframe).
        // Todo: As an improvement, create a struct Called OrderedHashSet<T>
        Some(old_price) if old_price.timestamp() < new_price.timestamp() => {
            entry.remove(new_price);
            entry.insert(new_price.clone());
        }

        // Replace time frame of new price with old value.
        Some(old_price) => new_price.set_timestamp(*old_price.timestamp()),
    }
}

impl Default for RateGraph {
    fn default() -> Self {
        Self {
            exchange_prices: HashMap::new(),
            graph: DiGraphMap::new(),
            next: DiGraphMap::default(),
            rate: DiGraphMap::default(),
        }
    }
}

impl From<Vec<PriceUpdate>> for RateGraph {
    fn from(prices: Vec<PriceUpdate>) -> RateGraph {
        let mut graph = Self {
            exchange_prices: HashMap::new(),
            graph: DiGraphMap::new(),
            next: DiGraphMap::default(),
            rate: DiGraphMap::default(),
        };

        graph.insert(prices);
        graph
    }
}

fn decimal_to_f64(from: Decimal) -> f64 {
    let decimal = from.to_string();
    let floating_value: f64 = decimal.parse().unwrap();
    floating_value
}
