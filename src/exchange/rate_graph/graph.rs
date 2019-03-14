use crate::exchange::Path;
use crate::utils::map_utils::update_with_recent;
use crate::{
    exchange::{Currency, ExchangeVertex, ExchangeVertexPair, FullPath},
    input::{ExchangeRateRequest, PriceUpdate},
};
use chrono::NaiveDateTime;
use itertools::Itertools;
use petgraph::graphmap::DiGraphMap;
use rust_decimal::Decimal;
use std::{collections::HashMap, iter::FromIterator};

/// Errors that occur while utilizing a rate graph.
#[derive(Debug)]
pub enum RateGraphError {
    /// The specified exchange vertex doesn't exist in the graph.
    UnknownExchange(ExchangeVertex),

    /// There exists no connection between vertex pairs in thr graphs.
    NoEdgesBetweenNodes(ExchangeVertex, ExchangeVertex),
}

/// An time ordered exchange graph that pairs exchange vertex to the time they
/// are created (i.e. their timestamp).
///
/// Upon the insert of a new exchange vertex, if the new time value
/// is more recent than the previous, update the time for the given vertex,
/// or else create the vertex for the first time.
type RecentExchangeMap = HashMap<ExchangeVertex, NaiveDateTime>;

/// A graph reresentation of all exchanges connected together by a
/// specified weight as the rate if transactions.
#[derive(Debug)]
pub struct RateGraph {
    exchange_prices: HashMap<Currency, RecentExchangeMap>,
    graph: DiGraphMap<ExchangeVertex, Decimal>,
    path: Path<ExchangeVertex, Decimal>,
}

impl RateGraph {
    /// Get the inner graph of a rate graph.
    pub fn graph(&self) -> &DiGraphMap<ExchangeVertex, Decimal> {
        &self.graph
    }

    /// Calculate the best possible echange rates using the Floyd Warshall's
    /// algorithm.
    fn calculate_best_rates(&mut self) {
        self.path.floyd_warshall(&self.graph, 0);
    }

    /// Get all the node pairs with edges that exist for every currency in the graph.
    fn currency_nodes(&self) -> Vec<ExchangeVertexPair> {
        self.exchange_prices
            .iter()
            .flat_map(|currency| {
                currency
                    .1
                    .iter()
                    .combinations(2)
                    .map(|e| (*e[0].0, *e[1].0))
            })
            .collect()
    }

    /// Add a bidirectional edge connecting `src` and `dst` to the graph,
    /// with associated data weight. For a directed graph.
    ///
    /// Inserts nodes `src` and/or `dst` if they aren't already part of the graph.
    ///
    fn add_edge(
        &mut self,
        src: ExchangeVertex,
        dst: ExchangeVertex,
        forward_factor: Decimal,
        backward_factor: Decimal,
    ) {
        self.graph.add_edge(src, dst, forward_factor);
        self.graph.add_edge(dst, src, backward_factor);
        self.graph.add_edge(src, src, 0.into());
        self.graph.add_edge(dst, dst, 0.into());
    }

    /// Returns the most optimal sequence of trades and transfers across exchanges
    /// for the specified exchange rate request.
    pub fn full_path(
        &self,
        request: &ExchangeRateRequest,
    ) -> Result<FullPath<ExchangeVertex>, RateGraphError> {
        let (u, v) = ExchangeVertexPair::from(request);
        self.path
            .full_path(u, v)
            .map_err(|_| RateGraphError::NoEdgesBetweenNodes(u, v))
    }

    /// Insert a given collection of price updates into the a rate map.
    /// On insert, the best rates are immediately calculates.
    pub fn insert(&mut self, updates: impl IntoIterator<Item = PriceUpdate>) {
        for update in updates {
            let (ex1, ex2) = self.insert_price_update(&update);
            let src_node = self.graph.add_node(ex1);
            let dst_node = self.graph.add_node(ex2);

            self.add_edge(
                src_node,
                dst_node,
                update.forward_factor(),
                *update.backward_factor(),
            );
        }

        self.currency_nodes().iter().for_each(|pair| {
            self.add_edge(pair.0, pair.1, 1.into(), 1.into());
        });

        self.calculate_best_rates();
    }

    /// Add a price update to available exchange prices. If the price is not the
    /// latest, it becomes discarded. If it doesnt exist, it gets created.
    fn insert_price_update(&mut self, price: &PriceUpdate) -> ExchangeVertexPair {
        let (ex1, ex2) = ExchangeVertexPair::from(price);

        self.exchange_prices
            .entry(*ex1.currency())
            .and_modify(|map| update_with_recent(map, ex1, &price.timestamp()))
            .or_insert_with(|| HashMap::from_iter(vec![(ex1, *price.timestamp())]));

        self.exchange_prices
            .entry(*ex2.currency())
            .and_modify(|map| update_with_recent(map, ex2, &price.timestamp()))
            .or_insert_with(|| HashMap::from_iter(vec![(ex2, *price.timestamp())]));

        (ex1, ex2)
    }
}

impl Default for RateGraph {
    // Create a default rate graph, pre-filled with default initial values.
    fn default() -> Self {
        Self {
            exchange_prices: HashMap::new(),
            graph: DiGraphMap::new(),
            path: Path::default(),
        }
    }
}

impl From<Vec<PriceUpdate>> for RateGraph {
    /// Create a rate graph from a given vector of price updates.
    /// Only updates with the latest timestamp are considered during graph creation.
    fn from(prices: Vec<PriceUpdate>) -> RateGraph {
        let mut graph = Self {
            exchange_prices: HashMap::new(),
            graph: DiGraphMap::new(),
            path: Path::default(),
        };

        graph.insert(prices);
        graph
    }
}
