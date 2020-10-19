use crate::sampler::Sampler;

#[derive(Debug, Default)]
pub struct Builder {
    variable_degree: usize,
    constraint_degree: usize,
    scaling_factor: usize,
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

    pub fn scaling_factor(&mut self, factor: usize) -> &mut Self {
        self.scaling_factor = factor;
        self
    }

    pub fn build(&self) -> Sampler {
        Sampler {
            variable_degree: self.variable_degree,
            constraint_degree: self.constraint_degree,
            scaling_factor: self.scaling_factor,
        }
    }
}
