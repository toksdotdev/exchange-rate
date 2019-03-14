use crate::exchange::resources::{Currency, ExchangeType};

/// A exchange vertex representation of a given price update.
#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Clone, Copy, Ord)]
pub struct ExchangeVertex(ExchangeType, Currency);

/// ExchangeVertexPair comprising of a both (Source, Destination).
pub type ExchangeVertexPair = (ExchangeVertex, ExchangeVertex);

impl ExchangeVertex {
    /// Create a new `ExchangeVertex` from specified values.
    pub fn new(exchange: ExchangeType, currency: Currency) -> Self {
        Self(exchange, currency)
    }

    /// Gets the currency of an exchange.
    pub fn currency(&self) -> &Currency {
        &self.1
    }

    /// Gets the exchange type of an exchange.
    pub fn exchange(&self) -> &ExchangeType {
        &self.0
    }
}
