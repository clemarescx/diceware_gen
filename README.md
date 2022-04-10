# Diceware Gen

A CLI password generator based on the [Diceware](https://theworld.com/~reinhold/diceware.html) word list.

## Usage

Enter the number of words to generate when prompted and press enter.

## Information

Uses the [rand](https://crates.io/crates/rand) crate to simulate the dice rolls used to pick words from the list.
According to the crate's documentation, `thread_rng()` provides a cryptographically secure RNG.
