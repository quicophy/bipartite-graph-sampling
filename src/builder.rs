use crate::sampler::Sampler;

#[derive(Debug, Default)]
pub struct Builder {
    variable_degree: usize,
    constraint_degree: usize,
    number_of_variables: usize,
    number_of_constraints: usize,
}

impl Builder {
    pub fn variable_degree(&mut self, degree: usize) -> &mut Self {
        self.variable_degree = degree;
        self
    }

    pub fn constraint_degree(&mut self, degree: usize) -> &mut Self {
        self.constraint_degree = degree;
        self
    }

    pub fn number_of_variables(&mut self, n: usize) -> &mut Self {
        self.number_of_variables = n;
        self
    }

    pub fn number_of_constraints(&mut self, n: usize) -> &mut Self {
        self.number_of_constraints = n;
        self
    }

    pub fn build(&self) -> Sampler {
        Sampler {
            variable_degree: self.variable_degree,
            constraint_degree: self.constraint_degree,
            number_of_variables: self.number_of_variables,
            number_of_constraints: self.number_of_constraints,
        }
    }
}
