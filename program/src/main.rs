//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use sudoku_lib::sudoku;
use sudoku_lib::sudoku::is_valid_sudoku_solution;

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let initial_state = sp1_zkvm::io::read::<String>();
    let solution = sp1_zkvm::io::read::<String>();
    // println!("Initial State: {}", initial_state);
    // println!("Solution: {}", solution);
    let is_valid = is_valid_sudoku_solution(initial_state, solution);

    assert_eq!(is_valid, true, "The sudoku solution is not valid");
}
