//! An helper to build sampler.

use crate::error::InvalidParameters;
use crate::sampler::Sampler;

/// A builder for samplers.
///
/// See [`Sampler::builder`](Sampler) for more details.
#[derive(Debug, Default)]
pub struct Builder {
    variable_degree: usize,
    constraint_degree: usize,
    number_of_variables: usize,
    number_of_constraints: usize,
}

impl Builder {
    /// Fixes the variable's degree. Default is 0.
    pub fn variable_degree(&mut self, degree: usize) -> &mut Self {
        self.variable_degree = degree;
        self
    }

    /// Fixes the constraint's degree. Default is 0.
    pub fn constraint_degree(&mut self, degree: usize) -> &mut Self {
        self.constraint_degree = degree;
        self
    }

    /// Fixes the number of variables. Default is 0.
    pub fn number_of_variables(&mut self, n: usize) -> &mut Self {
        self.number_of_variables = n;
        self
    }

    /// Fixes the number of constraints. Default is 0.
    pub fn number_of_constraints(&mut self, n: usize) -> &mut Self {
        self.number_of_constraints = n;
        self
    }

    /// Build a sampler or returns an error if the number of variables times their degree is not the same
    /// as the number of constraints times their degree.
    pub fn build(&self) -> Result<Sampler, InvalidParameters> {
        if self.number_of_variables * self.variable_degree
            != self.number_of_constraints * self.constraint_degree
        {
            Err(InvalidParameters {
                variable_degree: self.variable_degree,
                constraint_degree: self.constraint_degree,
                number_of_variables: self.number_of_variables,
                number_of_constraints: self.number_of_constraints,
            })
        } else {
            Ok(Sampler {
                variable_degree: self.variable_degree,
                constraint_degree: self.constraint_degree,
                number_of_variables: self.number_of_variables,
                number_of_constraints: self.number_of_constraints,
            })
        }
    }
}
