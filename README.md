# bigs

A BIpartite Graph Sampler!

It is fast. I don't have any benchmarks, but on my machine I can sample a 7 000 000 nodes graph in roughly 1 or 2 seconds. 
This come in two versions: a rust library and a command line tool (cli).

## Library

I will try my best to publish up-to-date version of the library on [crates.io](https://crates.io/crates/bigs).
You can use it like any other crates in your own project. 

## CLI 

### Installation

- If you don't have a rust compiler, [install it](https://www.rust-lang.org/tools/install). Don't worry, you don't need to know 
how to program in rust to use this tool.
- Clone this repository.
- That is it!

### Usage

If you follow in the installation steps, you should also have the rust build tool: cargo.
Then if you go into the `cli` directory, you can simply use bigs like this
```bash
cargo run -- [SOME ARGUMENTS]
```

For example, if you want a graph with 30 variables of degrees 5 and 15 constraints of degree 10, you can do
```bash
cargo run -- -n 30 -v 5 -m 15 -c 10 
```

If it is too slow, you can add the `--release` option to cargo. Like this
```bash
cargo run --release -- -n 30 -v 5 -m 15 -c 10 
```

If you want to see all the options, run
```bash
cargo run -- --help
```

### Pro tips

If you are annoyed by the fact that you always have to type `cargo run --release -- [SOME ARGUMENTS]`
and you would prefer to only type `big [SOME ARGUMENTS]`, here is what you need to do. (This work on Linux and MacOs, sorry for windows users)

- Compile the cli program: `cargo build --release`
- This should create the folder `target/release/` which contains the `bigs` executable.
- Add this folder to your path. For example, if using bash (or zsh),
add this line to your .bashrc (or .zshrc): `export PATH=path/to/target/release:$PATH`.
This is `export PATH=path_where_you_cloned/target/release:$PATH`.
-That is it! You can now run the bigs command anywhere and anytime. If it doesn't work, try 
restarting your terminal.

## Constributions

I would love that many people in the group contribute to this small project since I believe it can be useful to many.
Here is a list of possible improvements sorted by order of complexity. If you want to contribute, feel free to create a pull request.
Also, don't hesitate to reach to me if you have any question. Finaly, if you have a cool idea to improve this software, you can create an new issue with 
your idea and we can discuss it. If it is indeed an awesome idae, I will add it to the list of possible contributions or you could implement it.
Thank you!

### Easy
- Correct grammatical errors in this README.
- Update code documentation.
- Document the output format.
- Improve error handling.

### Medium
- Add some tests. Right now, I create this project by merging bits of tested softwares that I have, but there is no test in this project.
- Implement wrapper for python or any other programming language. 

### Hard
- Add the possibility to fix the girth of the graph.
