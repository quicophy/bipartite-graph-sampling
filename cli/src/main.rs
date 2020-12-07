use bigs::error::InvalidParameters;
use bigs::graph::Graph;
use bigs::Sampler;
use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::Serialize;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bigs")]
/// The BIpartite Graph Sampler
struct Options {
    /// The number of constraints connected to each variable.
    #[structopt(
        short = "v",
        long = "vardegree",
        default_value = "3",
        name = "variable degree"
    )]
    variable_degree: usize,

    /// The number of variables connected to each constraint.
    #[structopt(
        short = "c",
        long = "constdegree",
        default_value = "3",
        name = "constraint degree"
    )]
    constraint_degree: usize,

    /// The number of variables in the graph.
    #[structopt(
        short = "n",
        long = "numvar",
        default_value = "3",
        name = "number of variables"
    )]
    number_of_variables: usize,

    /// The number of variables in the graph.
    #[structopt(
        short = "m",
        long = "numconst",
        default_value = "3",
        name = "number of constraints"
    )]
    number_of_constraints: usize,

    /// If prodived, the random number generator will be initialized with the given seed.
    /// Else a random seed will be used. If using the same seed and version of bigs, the
    /// generated graph is always the same.
    #[structopt(short = "r", long = "rngseed", name = "rng seed")]
    rng_seed: Option<u64>,

    /// If provided, the results will be saved at the specified location. Else,
    /// it will be printed to the standard output.
    #[structopt(short = "o", long = "output", parse(from_os_str), name = "output path")]
    output_path: Option<PathBuf>,
}

fn main() {
    let mut options = Options::from_args();
    let sampler = sampler(&options);
    match sampler {
        Ok(sampler) => {
            let mut rng = rng(&mut options);
            let graph = sampler.sample_with(&mut rng);
            save_or_display(graph, options);
        }
        Err(error) => {
            println!("Can't build a regular graph since n * v != m * c.");
            println!("n = {} (number of variables)", error.number_of_variables);
            println!("v = {} (variable's degree)", error.variable_degree);
            println!(
                "m = {} (number of constraints)",
                error.number_of_constraints
            );
            println!("c = {} (constraint's degree)", error.constraint_degree);
        }
    }
}

fn sampler(options: &Options) -> Result<Sampler, InvalidParameters> {
    Sampler::builder()
        .variable_degree(options.variable_degree)
        .constraint_degree(options.constraint_degree)
        .number_of_variables(options.number_of_variables)
        .number_of_constraints(options.number_of_constraints)
        .build()
}

fn rng(options: &mut Options) -> ChaCha20Rng {
    if options.rng_seed.is_none() {
        options.rng_seed = Some(thread_rng().gen());
    }
    ChaCha20Rng::seed_from_u64(options.rng_seed.unwrap())
}

fn save_or_display(graph: Graph, options: Options) {
    let output = Output {
        number_of_variables: graph.number_of_variables(),
        number_of_constraints: graph.number_of_constraints(),
        variable_degree: options.variable_degree,
        constraint_degree: options.constraint_degree,
        rng_seed: options.rng_seed.unwrap(),
        graph,
    };
    if let Some(path) = options.output_path {
        if let Ok(json) = serde_json::to_vec(&output) {
            match std::fs::write(&path, json) {
                Ok(_) => println!("Saved output to {}", path.to_string_lossy()),
                Err(e) => println!("Error while saving: {}", e),
            }
        } else {
            println!("Failed to convert to json");
            println!("{}", output);
        }
    } else {
        println!("{}", output);
    }
}

#[derive(Serialize)]
struct Output {
    number_of_variables: usize,
    number_of_constraints: usize,
    variable_degree: usize,
    constraint_degree: usize,
    rng_seed: u64,
    graph: Graph,
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Random graph\n============\n")?;
        writeln!(f, "Number of variables: {}", self.number_of_variables)?;
        writeln!(f, "Number of constraints: {}", self.number_of_constraints)?;
        writeln!(f, "Variable degree: {}", self.variable_degree)?;
        writeln!(f, "Constraint degree: {}", self.constraint_degree)?;
        writeln!(f, "Rng seed: {}\n", self.rng_seed)?;
        writeln!(f, "Graph\n-----\n{:?}", self.graph)?;
        Ok(())
    }
}
