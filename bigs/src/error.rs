use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InvalidParameters {
    pub number_of_variables: usize,
    pub number_of_constraints: usize,
    pub variable_degree: usize,
    pub constraint_degree: usize,
}

impl fmt::Display for InvalidParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!(
            "can't sample a graph with {} variables of degree {} and {} constraints of degree {}",
            self.number_of_variables,
            self.variable_degree,
            self.number_of_constraints,
            self.constraint_degree
        )
        .fmt(f)
    }
}

impl Error for InvalidParameters {}
