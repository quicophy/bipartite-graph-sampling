use crate::sampler::Sampler;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Edge {
    pub variable: usize,
    pub constraint: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Graph {
    variable_neighbors: Vec<IndexSet<usize>>,
    constraint_neighbors: Vec<IndexSet<usize>>,
    edges: IndexSet<Edge>,
}

impl Graph {
    pub fn from_sampler(sampler: &Sampler) -> Self {
        Self {
            variable_neighbors: vec![
                IndexSet::with_capacity(sampler.variable_degree());
                sampler.number_of_variables()
            ],
            constraint_neighbors: vec![
                IndexSet::with_capacity(sampler.constraint_degree());
                sampler.number_of_constraints()
            ],
            edges: IndexSet::with_capacity(sampler.number_of_edges()),
        }
    }

    pub fn contains(&self, edge: Edge) -> bool {
        self.edges.contains(&edge)
    }

    pub fn insert(&mut self, edge: Edge) -> bool {
        if self.edges.insert(edge) {
            self.variable_neighbors[edge.variable].insert(edge.constraint);
            self.constraint_neighbors[edge.constraint].insert(edge.variable);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, edge: Edge) -> bool {
        if self.edges.remove(&edge) {
            self.variable_neighbors[edge.variable].remove(&edge.constraint);
            self.constraint_neighbors[edge.constraint].remove(&edge.variable);
            true
        } else {
            false
        }
    }

    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.iter().cloned()
    }

    pub fn number_of_variables(&self) -> usize {
        self.variable_neighbors.len()
    }

    pub fn number_of_constraints(&self) -> usize {
        self.constraint_neighbors.len()
    }

    pub fn number_of_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn variable_neighbors(&self) -> impl Iterator<Item = &IndexSet<usize>> {
        self.variable_neighbors.iter()
    }

    pub fn constraint_neighbors(&self) -> impl Iterator<Item = &IndexSet<usize>> {
        self.constraint_neighbors.iter()
    }
}
