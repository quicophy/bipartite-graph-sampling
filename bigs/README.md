# bigs

A BIpartite Graph Sampler

A tool to generate regular bipartite graphs.
A bipartite graph is a set of variables and constraints 
(named like this because of SAT problems)
together with a set of edges.
Right now, only regular graphs can be sampled.
That is, graphs with the same degree for all variables and the same for all constraints.

## Example

```rust
use bigs::Sampler;
use rand::thread_rng;

let sampler = Sampler::builder()
    .number_of_variables(10)
    .number_of_constraints(6)
    .variable_degree(3)
    .constraint_degree(5)
    .build();

let graph = sampler.sample_with(&mut thread_rng());
let other_graph = sampler.sample_with(&mut thread_rng());
```
