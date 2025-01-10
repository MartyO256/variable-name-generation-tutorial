# Variable Name Generation Tutorial - Implementation

This directory contains the Rust implementation of the variable name generation algorithm for untyped lambda expressions in mixed representation.

## Prerequisites

Before building or running the project, ensure you have the following:

- [Rust](https://www.rust-lang.org/) (the project was implemented and tested using version 1.80.1)
- Cargo (comes with Rust installation)

## Building

1. Clone the repository:

```sh
git clone https://github.com/MartyO256/variable-name-generation-tutorial.git
cd variable-name-generation-tutorial/implementation
```

2. Build the project using Cargo:

```sh
cargo build
```

## Testing

Run the tests using Cargo:

```sh
cargo test
```

## Project Structure

The main files implementing variable name generation are listed in the following table:

| Source File                             | Description                                                                       |
| --------------------------------------- | --------------------------------------------------------------------------------- |
| `expression.rs`                         | Definition for the expression abstract syntax tree and expression arenas          |
| `strings.rs`                            | Definition for interned strings in string arenas                                  |
| `to_named.rs`                           | Conversion from expressions in mixed representation to fully named representation |
| `admissible_variable_name_generator.rs` | Generation of variable names using streams and admissibility predicates           |

The remaining files contain various utilities to support testing of the variable name generation algorithm, as well as toy functions to demonstrate how to use expression arenas.

| Source File                      | Description                                                        |
| -------------------------------- | ------------------------------------------------------------------ |
| `expression_size.rs`             | Counting the number of nodes in an expression AST                  |
| `expression_height.rs`           | Computing the height of an expression AST                          |
| `expression_parent.rs`           | Computing the association between parent and child nodes           |
| `expression_free_variables.rs`   | Computing the set of free variables occurring in an expression AST |
| `referencing_environment.rs`     | Data structure to represent the state of identifiers in scope      |
| `equality.rs`                    | Structural equality predicate for expression ASTs                  |
| `alpha_equivalence.rs`           | Alpha-equivalence predicate for expression ASTs                    |
| `expression_locally_nameless.rs` | Predicate for locally nameless expression ASTs                     |
| `expression_named.rs`            | Predicate for fully named expression ASTs                          |
| `parser.rs`                      | Recursive-descent parser for expressions                           |
| `pretty_print.rs`                | Recursive-descent pretty-printer for expressions                   |
| `random_expressions.rs`          | Random expression sampler for fuzzing                              |
