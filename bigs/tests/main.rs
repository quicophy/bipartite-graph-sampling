use bigs::Sampler;
use rand::rngs::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};

const NUMBER_OF_RANDOM_TESTS: u32 = 10;

#[test]
fn graphs_have_the_right_parameters() {
    let sampler = Sampler::builder()
        .number_of_variables(10)
        .number_of_constraints(8)
        .variable_degree(4)
        .constraint_degree(5)
        .build();
    for _ in 0..NUMBER_OF_RANDOM_TESTS {
        let graph = sampler.sample_with(&mut thread_rng());

        assert_eq!(graph.number_of_variables(), 10);
        assert_eq!(graph.number_of_constraints(), 8);
        assert_eq!(graph.number_of_edges(), 40);

        for neighbors in graph.variable_neighbors() {
            assert_eq!(neighbors.len(), 4);
        }

        for neighbors in graph.constraint_neighbors() {
            assert_eq!(neighbors.len(), 5);
        }
    }
}

#[test]
fn sampling_is_reproducable() {
    let seed = thread_rng().gen();
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut other_rng = SmallRng::seed_from_u64(seed);

    let sampler = Sampler::builder()
        .number_of_variables(9)
        .number_of_constraints(15)
        .variable_degree(5)
        .constraint_degree(3)
        .build();

    for _ in 0..NUMBER_OF_RANDOM_TESTS {
        let graph = sampler.sample_with(&mut rng);
        let other_graph = sampler.sample_with(&mut other_rng);
        assert_eq!(graph, other_graph);
    }
}

#[test]
#[should_panic]
fn panic_if_parameters_do_not_fit() {
    Sampler::builder()
        .number_of_variables(10)
        .number_of_constraints(10)
        .variable_degree(3)
        .constraint_degree(2)
        .build();
}
