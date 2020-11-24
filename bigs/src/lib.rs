//! BIpartite Graph Sampler.
//!
//! A tool to generate regular bipartite graphs.
//! A bipartite graph is a set of variables and constraints (named like this because of SAT problems)
//! together with a set of edges.
//! Right now, only regular graphs can be sampled.
//! That is, graphs with the same degree for all variables and the same for all constraints.
//!
//! # Quick start
//!
//! Graphs are sampled using a [`Sampler`](crate::Sampler) which are instanciated via
//! the [`builder`](crate::Sampler::builder) method.
//! Then, any random number generator can be used to sample a [`Graph`](crate::Graph).
//!
//! ```
//! use bigs::Sampler;
//! use rand::thread_rng;
//!
//! let sampler = Sampler::builder()
//!     .number_of_variables(10)
//!     .number_of_constraints(6)
//!     .variable_degree(3)
//!     .constraint_degree(5)
//!     .build();
//!
//! let graph = sampler.sample_with(&mut thread_rng());
//! let other_graph = sampler.sample_with(&mut thread_rng());
//! ```

pub mod builder;
pub mod graph;
pub mod sampler;

pub use graph::Graph;
pub use sampler::Sampler;
