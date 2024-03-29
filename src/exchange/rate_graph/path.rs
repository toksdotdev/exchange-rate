use petgraph::{graphmap::NodeTrait, prelude::DiGraphMap, visit::IntoEdgeReferences};
use std::ops::Add;
use std::{fmt::Debug, ops::Mul};

pub type FullPath<T> = Vec<T>;
pub type PathCost<T> = T;

#[derive(Debug)]
pub struct Path<V, E>
where
    V: NodeTrait,
    E: NodeTrait,
{
    rate: DiGraphMap<V, E>,
    next: DiGraphMap<V, Option<V>>,
}

impl<V, E> Path<V, E>
where
    V: NodeTrait + Debug,
    E: NodeTrait + Debug + From<i32> + Copy + Add<Output = E>,
{
    /// Gets the full most optimal path for moving from a given source node (`u`)
    /// to a given destination node (`v`).
    pub fn full_path(&self, mut u: V, v: V) -> Option<(FullPath<V>, PathCost<E>)> {
        if !self.next.contains_edge(u, v) {
            return None;
        }

        let mut path = vec![u];
        let mut path_cost: E = 0.into();
        while u != v {
            if let Some(node) = self.next.edge_weight(u, v).unwrap() {
                path_cost = path_cost + *self.rate.edge_weight(u, v).unwrap();
                u = *node;
                path.push(u);
            }
        }

        Some((path, path_cost))
    }

    /// Populate the path graph with default values, by using the
    /// nodes of an existing directional graph.
    fn populate_from_graph<D>(&mut self, g: &DiGraphMap<V, E>, default: D)
    where
        D: Into<E> + Copy,
    {
        for x in g.nodes() {
            for y in g.nodes() {
                self.rate.add_edge(x, y, default.into());
                self.next.add_edge(x, y, None);
            }
        }
    }

    /// Calculate the shortest path between all vertices using the Floyd
    /// Warshall's Algorithm.
    pub fn floyd_warshall<D>(&mut self, g: &DiGraphMap<V, E>, default: D)
    where
        E: Mul<Output = E> + Copy,
        D: Into<E> + Copy,
    {
        self.populate_from_graph(g, default.into());

        for (u, v, w) in g.edge_references() {
            self.rate.add_edge(u, v, *w);
            self.next.add_edge(u, v, Some(v));
        }

        for k in g.nodes() {
            for i in g.nodes() {
                for j in g.nodes() {
                    // Safe to unwrap, as edges in graph g have been
                    // populated into both rate and next graph respectively.
                    let i_j: E = *self.rate.edge_weight(i, j).unwrap();
                    let total = (*self.rate.edge_weight(i, k).unwrap())
                        * (*self.rate.edge_weight(k, j).unwrap());

                    if i_j < total {
                        let weight = *self.next.edge_weight(i, k).unwrap();
                        self.rate.add_edge(i, j, total);
                        self.next.add_edge(i, j, weight);
                    }
                }
            }
        }
    }
}

impl<V, E> Default for Path<V, E>
where
    V: NodeTrait,
    E: NodeTrait,
{
    /// Create a default path object.
    fn default() -> Self {
        Self {
            next: DiGraphMap::default(),
            rate: DiGraphMap::default(),
        }
    }
}
