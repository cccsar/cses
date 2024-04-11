# cses
Implementation of [cses](!https://cses.fi/problemset/) problems. But in Rust

## Organization

Problems are organized by category under `src/` directories as submodules.

Every new solution is given `problem_name.rs` under it's respective submodule and must be re-exported on the propper `mod.rs` file in order to be visible from `main.rs`

`tcs/` directory holds test cases and follows the same hierarchical structure of submodules of `src/` (based on problem cathegory).

`submissions/` simply holds the rust file to be submitted. Testing of output here goes like

```{bash}
    rustc -o out output.rs && ./out < ../tc/some_cat/some_tc.txt
```