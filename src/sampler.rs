use crate::builder::Builder;
use crate::graph::{Edge, Graph};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Sampler {
    pub(crate) variable_degree: usize,
    pub(crate) constraint_degree: usize,
    pub(crate) scaling_factor: usize,
}

impl Sampler {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn sample_with<R: Rng>(&self, rng: &mut R) -> Graph {
        Sample::from_sampler_and_rng(self, rng).generate()
    }

    pub fn number_of_variables(&self) -> usize {
        self.constraint_degree * self.scaling_factor
    }

    pub fn number_of_constraints(&self) -> usize {
        self.variable_degree * self.scaling_factor
    }

    pub fn number_of_edges(&self) -> usize {
        self.variable_degree * self.constraint_degree * self.scaling_factor
    }

    pub fn variable_degree(&self) -> usize {
        self.variable_degree
    }

    pub fn constraint_degree(&self) -> usize {
        self.constraint_degree
    }

    pub fn scaling_factor(&self) -> usize {
        self.scaling_factor
    }
}

struct Sample<'s> {
    sampler: &'s Sampler,
    candidate_edges: VecDeque<Edge>,
}

impl<'s> Sample<'s> {
    fn from_sampler_and_rng<R: Rng>(sampler: &'s Sampler, rng: &mut R) -> Self {
        Self {
            sampler,
            candidate_edges: Self::candidate_edges(sampler, rng),
        }
    }

    fn candidate_edges<R: Rng>(sampler: &Sampler, rng: &mut R) -> VecDeque<Edge> {
        Self::candidate_variables(sampler, rng)
            .zip(Self::candidate_constraints(sampler, rng))
            .map(|(variable, constraint)| Edge {
                variable,
                constraint,
            })
            .collect()
    }

    fn candidate_variables<R: Rng>(sampler: &Sampler, rng: &mut R) -> impl Iterator<Item = usize> {
        let mut variables = (0..sampler.number_of_variables())
            .flat_map(|variable| std::iter::repeat(variable).take(sampler.variable_degree()))
            .collect::<Vec<usize>>();
        variables.shuffle(rng);
        variables.into_iter()
    }

    fn candidate_constraints<R: Rng>(
        sampler: &Sampler,
        rng: &mut R,
    ) -> impl Iterator<Item = usize> {
        let mut constraints = (0..sampler.number_of_constraints())
            .flat_map(|constraint| std::iter::repeat(constraint).take(sampler.constraint_degree()))
            .collect::<Vec<usize>>();
        constraints.shuffle(rng);
        constraints.into_iter()
    }

    fn generate(mut self) -> Graph {
        let mut graph = Graph::from_sampler(self.sampler);
        while let Some(edge) = self.candidate_edges.pop_front() {
            if graph.contains(edge) {
                self.try_to_swap_edge_and_insert(&mut graph, edge);
            } else {
                graph.insert(edge);
            }
        }
        graph
    }

    fn try_to_swap_edge_and_insert(&mut self, graph: &mut Graph, edge: Edge) {
        if let Some(edge_to_swap) = Self::find_edge_to_swap(edge, &graph) {
            graph.remove(edge_to_swap);
            let (first_swapped_edge, second_swapped_edge) = Self::swap(edge, edge_to_swap);
            graph.insert(first_swapped_edge);
            graph.insert(second_swapped_edge);
        } else {
            self.candidate_edges.push_back(edge);
        }
    }

    fn find_edge_to_swap(target_edge: Edge, graph: &Graph) -> Option<Edge> {
        graph.edges().find(|edge| {
            let (first_swapped_edge, second_swapped_edge) = Self::swap(*edge, target_edge);
            !graph.contains(first_swapped_edge) && !graph.contains(second_swapped_edge)
        })
    }

    fn swap(first_edge: Edge, second_edge: Edge) -> (Edge, Edge) {
        (
            Edge {
                variable: first_edge.variable,
                constraint: second_edge.constraint,
            },
            Edge {
                variable: second_edge.variable,
                constraint: first_edge.constraint,
            },
        )
    }
}
