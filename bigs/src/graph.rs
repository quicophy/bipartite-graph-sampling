//! Everything related to graphs: edges, nodes and, of course, graphs.
//!
//! The most important part of this module is [`Graph`](Graph).
//! However, if you want to manually build graphs,
//! you will need to use [`Edge`](Edge).

use crate::Sampler;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

/// A (variable, constraint) pair.
///
/// Since variables and constraints are different sets of nodes,
/// it is possible to have an edge with the same value for variable and constraint.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Edge {
    pub variable: usize,
    pub constraint: usize,
}

impl Edge {
    /// Creates a new edge for the given variable and constraint.
    pub fn new(variable: usize, constraint: usize) -> Self {
        Self {
            variable,
            constraint,
        }
    }
}

/// A bipartite regular graph.
///
/// A graph is a set of variables and constraints together with
/// a set of edges.
/// An edge is a (variable, constraint) pair.
///
/// A graph can be build manually or from a [`Sampler`](Sampler).
///
/// # Example
///
/// ```
/// use bigs::graph::{Edge, Graph};
///
/// let mut graph = Graph::new();
///
/// graph.insert_edge(Edge::new(0, 0)); // Edge between variable 0 and constraint 0.
/// graph.insert_edge(Edge::new(0, 1)); // Edge between variable 0 and constraint 1.
/// graph.insert_edge(Edge::new(1, 2)); // Edge between variable 1 and constraint 2.
/// graph.insert_edge(Edge::new(1, 3)); // Edge between variable 1 and constraint 3.
///
/// assert_eq!(graph.number_of_variables(), 2);
/// assert_eq!(graph.number_of_constraints(), 4);
/// assert_eq!(graph.number_of_edges(), 4);
///
/// for variable in graph.variables() {
///     assert_eq!(variable.degree(), 2);
/// }
///
/// for constraint in graph.constraints() {
///     assert_eq!(constraint.degree(), 1);
/// }
/// ```
///
/// # Performance tips
///
/// Don't use unnecessary large labels for variable and constraint.
/// The construction assume that you will use labels 0 to n - 1 if you
/// want n variables and the same for constraints.
///
/// If for whatever reason you want to assign a node with a large label,
/// notes that this will allocate the needed memory for all nodes up to that label.
///
/// ```
/// use bigs::graph::{Edge, Graph};
///
/// let mut graph = Graph::new();
/// graph.insert_edge(Edge::new(0, 42));
///
/// assert_eq!(graph.number_of_variables(), 1);
/// assert_eq!(graph.number_of_constraints(), 43);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Graph {
    variable_neighbors: Vec<IndexSet<usize>>,
    constraint_neighbors: Vec<IndexSet<usize>>,
    edges: IndexSet<Edge>,
}

impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Self {
            variable_neighbors: Vec::new(),
            constraint_neighbors: Vec::new(),
            edges: IndexSet::new(),
        }
    }

    /// Checks if the given edge is in the graph.
    pub fn contains_edge(&self, edge: Edge) -> bool {
        self.edges.contains(&edge)
    }

    /// Inserts the given edge in the graph and returns true if the
    /// edge was not already in the graph.
    ///
    /// If the edge variable is greater or equal to the number of variables in the graph,
    /// the number of variables will be incremented by the difference.
    /// Same holds for constraints.
    ///
    /// # Example
    ///
    /// ```
    /// use bigs::graph::{Edge, Graph};
    ///
    /// let mut graph = Graph::new();
    /// assert_eq!(graph.number_of_variables(), 0);
    /// assert_eq!(graph.number_of_constraints(), 0);
    /// assert_eq!(graph.number_of_edges(), 0);
    ///
    /// graph.insert_edge(Edge::new(0, 0));
    /// assert_eq!(graph.number_of_variables(), 1);
    /// assert_eq!(graph.number_of_constraints(), 1);
    /// assert_eq!(graph.number_of_edges(), 1);
    ///
    /// graph.insert_edge(Edge::new(5, 6));
    /// assert_eq!(graph.number_of_variables(), 6);
    /// assert_eq!(graph.number_of_constraints(), 7);
    /// assert_eq!(graph.number_of_edges(), 2);
    ///
    /// assert_eq!(graph.insert_edge(Edge::new(0, 0)), false);
    /// ```
    pub fn insert_edge(&mut self, edge: Edge) -> bool {
        if self.edges.insert(edge) {
            self.insert_variable(edge);
            self.insert_constraint(edge);
            true
        } else {
            false
        }
    }

    fn insert_variable(&mut self, edge: Edge) {
        if edge.variable >= self.number_of_variables() {
            self.variable_neighbors.extend(vec![
                IndexSet::new();
                edge.variable - self.number_of_variables() + 1
            ]);
        }
        self.variable_neighbors[edge.variable].insert(edge.constraint);
    }

    fn insert_constraint(&mut self, edge: Edge) {
        if edge.constraint >= self.number_of_constraints() {
            self.constraint_neighbors.extend(vec![
                IndexSet::new();
                edge.constraint - self.number_of_constraints()
                    + 1
            ]);
        }
        self.constraint_neighbors[edge.constraint].insert(edge.variable);
    }

    /// Removes the given edge from the graph if it exists and returns true.
    /// Else, returns false.
    ///
    /// However, this do not update the number of variables and constraints in the graph.
    ///
    /// # Example
    ///
    /// ```
    /// use bigs::graph::{Edge, Graph};
    ///
    /// let mut graph = Graph::new();
    /// assert_eq!(graph.number_of_variables(), 0);
    /// assert_eq!(graph.number_of_constraints(), 0);
    /// assert_eq!(graph.number_of_edges(), 0);
    ///
    /// graph.insert_edge(Edge::new(0, 0));
    /// assert_eq!(graph.number_of_variables(), 1);
    /// assert_eq!(graph.number_of_constraints(), 1);
    /// assert_eq!(graph.number_of_edges(), 1);
    ///
    /// graph.remove_edge(Edge::new(0, 0));
    /// assert_eq!(graph.number_of_variables(), 1);
    /// assert_eq!(graph.number_of_constraints(), 1);
    /// assert_eq!(graph.number_of_edges(), 0);
    /// ```
    pub fn remove_edge(&mut self, edge: Edge) -> bool {
        if self.edges.remove(&edge) {
            self.variable_neighbors[edge.variable].remove(&edge.constraint);
            self.constraint_neighbors[edge.constraint].remove(&edge.variable);
            true
        } else {
            false
        }
    }

    /// Returns an iterator over all edges in the graph in some possibly random order.
    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.iter().cloned()
    }

    /// Returns the number of variables in the graph.
    ///
    /// That is, the one more than the highest variable label inserted in the graph.
    pub fn number_of_variables(&self) -> usize {
        self.variable_neighbors.len()
    }

    /// Returns the number of constraints in the graph.
    ///
    /// That is, the one more than the highest constraint label inserted in the graph.
    pub fn number_of_constraints(&self) -> usize {
        self.constraint_neighbors.len()
    }

    /// Returns the number of edges in the graph.
    pub fn number_of_edges(&self) -> usize {
        self.edges.len()
    }

    /// Returns an iterator over all variables in the graph in increasing label order.
    ///
    /// # Example
    ///
    /// ```
    /// use bigs::graph::{Edge, Graph};
    /// use indexmap::indexset;
    ///
    /// let mut graph = Graph::new();
    ///
    /// graph.insert_edge(Edge::new(0, 0)); // Edge between variable 0 and constraint 0.
    /// graph.insert_edge(Edge::new(0, 1)); // Edge between variable 0 and constraint 1.
    /// graph.insert_edge(Edge::new(1, 2)); // Edge between variable 1 and constraint 2.
    /// graph.insert_edge(Edge::new(1, 3)); // Edge between variable 1 and constraint 3.
    ///
    /// let mut iter = graph.variables();
    ///
    /// let first_variable = iter.next().unwrap();
    /// assert_eq!(first_variable.label(), 0);
    /// assert_eq!(first_variable.degree(), 2);
    /// assert_eq!(first_variable.neighbors(), &indexset! { 0, 1 });
    ///
    /// let second_variable = iter.next().unwrap();
    /// assert_eq!(second_variable.label(), 1);
    /// assert_eq!(second_variable.degree(), 2);
    /// assert_eq!(second_variable.neighbors(), &indexset! { 2, 3 });
    ///
    /// assert!(iter.next().is_none());
    /// ```
    pub fn variables(&self) -> Nodes {
        Nodes {
            iter: self.variable_neighbors.iter().enumerate(),
            kind: NodeKind::Variable,
        }
    }

    /// Returns an iterator over all constraints in the graph in increasing label order.
    ///
    /// # Example
    ///
    /// ```
    /// use bigs::graph::{Edge, Graph};
    /// use indexmap::indexset;
    ///
    /// let mut graph = Graph::new();
    ///
    /// graph.insert_edge(Edge::new(0, 0)); // Edge between variable 0 and constraint 0.
    /// graph.insert_edge(Edge::new(0, 1)); // Edge between variable 0 and constraint 1.
    /// graph.insert_edge(Edge::new(1, 2)); // Edge between variable 1 and constraint 2.
    /// graph.insert_edge(Edge::new(1, 3)); // Edge between variable 1 and constraint 3.
    ///
    /// let mut iter = graph.constraints();
    ///
    /// let first_constraint = iter.next().unwrap();
    /// assert_eq!(first_constraint.label(), 0);
    /// assert_eq!(first_constraint.degree(), 1);
    /// assert_eq!(first_constraint.neighbors(), &indexset! { 0 });
    ///
    /// let second_constraint = iter.next().unwrap();
    /// assert_eq!(second_constraint.label(), 1);
    /// assert_eq!(second_constraint.degree(), 1);
    /// assert_eq!(second_constraint.neighbors(), &indexset! { 0 });
    ///
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn constraints(&self) -> Nodes {
        Nodes {
            iter: self.constraint_neighbors.iter().enumerate(),
            kind: NodeKind::Constraint,
        }
    }

    pub(crate) fn from_sampler(sampler: &Sampler) -> Self {
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
}

/// An iterator for a set of nodes in a graph.
///
/// This is created via the [`Graph::variables`](Graph::variables)
/// or the [`Graph::constraints`](Graph::constraints) methods.
pub struct Nodes<'g> {
    iter: std::iter::Enumerate<std::slice::Iter<'g, IndexSet<usize>>>,
    kind: NodeKind,
}

impl<'g> Iterator for Nodes<'g> {
    type Item = Node<'g>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(label, neighbors)| Node {
            neighbors,
            label,
            kind: self.kind,
        })
    }
}

/// A node in the graph.
///
/// This is used to iterates throught the nodes of a graph.
pub struct Node<'g> {
    neighbors: &'g IndexSet<usize>,
    label: usize,
    kind: NodeKind,
}

impl<'g> Node<'g> {
    /// Returns the set of labels of the neighbors of the node.
    pub fn neighbors(&self) -> &IndexSet<usize> {
        self.neighbors
    }

    /// Returns the label of the node.
    pub fn label(&self) -> usize {
        self.label
    }

    /// Returns the degree of the node.
    /// That is, the number of neighbors.
    pub fn degree(&self) -> usize {
        self.neighbors.len()
    }

    /// Checks if a node a neighbor with the given label
    pub fn has_neighbor(&self, label: usize) -> bool {
        self.neighbors.contains(&label)
    }

    /// Checks if a node is a variable.
    pub fn is_variable(&self) -> bool {
        self.kind == NodeKind::Variable
    }

    /// Checks if a node is a constraint.
    pub fn is_constraint(&self) -> bool {
        self.kind == NodeKind::Constraint
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeKind {
    Variable,
    Constraint,
}
