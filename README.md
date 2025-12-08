# Conway's Game of Life GACNNs

**GA-optimized CNNs for predicting Conway's Game of Life**

![Rust 1.84.0 badge](https://img.shields.io/badge/Rust-1.84.0-E33B26)

This repository contains the WIP evolutionary algorithm and (convolutional) neural network crates for optimizing and training CNNs using GAs, as well as a crate for producing a dataset of 8×8 toroidal initial and n-timestep-future boards of Conway's Game of Life, which would be used for training a GACNN to predict the future state of any board given any number of timesteps to predict forward.

---

[Getting Started](#getting-started)  
[Prerequisites and Dependencies](#prerequisites-and-dependencies)  
[Installation](#installation)  

[Usage](#usage)  
[Features](#features)  
[Architecture Overview](#architecture-overview)  

[Contribution](#contribution)  

[License](#license)  

---

## Getting Started

### Prerequisites and Dependencies

All code has been written from scratch in Rust using only the core and standard libraries, plus the [rand crate](https://docs.rs/rand/latest/rand/index.html). Future versions of this project may have additional requirements for using GPGPU acceleration.

### Installation

This project and all of its crates are build/runnable via Cargo once the repository has been cloned.


## Usage

### Features

This project provides the `dataset-generator`, `evolutionary-algorithm`, and `neural-net` Rust crates. The `dataset-generator` crate supplies a CLI program for generating a dataset of initial and n-timestep future toroidal square Conway's Game of Life boards; the `evolutionary-algorithm` and `neural-net` crates supply frameworks for creating evolutionary/genetic algorithms and neural networks, respectively.

**DISCLAIMERS:**  
- The `neural-net` Rust crate is in very early WIP and not currently usable; it is currently only found in the `neural-network` branch  
- The `evolutionary-algorithm` Rust crate framework is complete in the `main` branch, but is lacking many default implementations which are desired in a full library release  
- The `dataset-generator` Rust crate is complete in the `main` branch, but has very narrow scope for generating only randomly-initialized toroidal Conway's Game of Life boards of a constant size and projecting them a range-random number of timesteps into the future  

### Architecture Overview

This project contains two library crates (`evolutionary-algorithm` and `neural-net`) and one binary crate (`dataset-generator`). Once the GA and CNN frameworks/libraries are sufficiently fleshed-out, a new binary crate will be created for the purpose of training and researching the use of GACNN for predicting 8×8 toroidal boards of Conway's Game of Life any number of timesteps into the future. Details on the library crates are provided below.

#### Evolutionary Algorithm Crate

**Evolution module:** `evolution.rs`; provides the core struct and traits which execute an evolutionary algorithm  
- Struct `Evolution`: generic over type `T`, which is the genotype type  
	- `generation: Vec<T>`: current population of `T` genotypes  
	- `fn new(population_size: usize) -> Result<Evolution<T>, ()>`: generic over type `Initializer`, which is the `GenerationInitializer` to use to generate the initial population. Returns `Err` for 0 `population_size` or `Ok(Evolution)` otherwise  
	- `fn new_from(generation: Vec<T>) -> Result<Evolution<T>, ()>`: creates a new `Evolution` from the given initial population. Returns `Err` for an empty `generation` or `Ok(Evolution)` otherwise  
	- `fn solutions(&self) -> &[T]`: returns a view into the current population  
	- `fn evolve(&mut self) -> usize`: generic over types `Convergence`, `Crossover`, `Fitness`, `MateSelections`, `Mutation`, `Reproductions`, and `Selection`, which are the `EvolutionConvergenceChecker`, `Recombinator`, `FitnessEvaluator`, `Matcher`, `Mutator`, `Reproducer`, and `Selector` to use in the evolution process. Returns the number of generations it took to reach convergence  
- Trait `EvolutionConvergenceChecker`: generic over type `T`, which is the genotype type  
	- `fn is_converged(generations: usize, solutions: &[T]) -> bool`: returns whether the evolutionary/genetic algorithm has converged based on the number of generations elapsed and the current population  
- Trait `GenerationInitializer`: generic over type `T`, which is the genotype type  
	- `fn initialize(population_size: usize) -> Vec<T>`: returns an initial population of size `population_size`  

**Fitness Evaluator module:** `fitness_evaluator.rs`; provides the trait for evaluating genotype fitness  
- Trait `FitnessEvaluator`: generic over type `T`, which is the genotype type  
	- `fn evaluate(individual: &T) -> f64`: returns the fitness score of `individual`  

**Matcher module:** `matcher.rs`; provides the trait for matching parent genotypes for recombination  
- Trait `Matcher`: generic over type `T`, which is the genotype type  
	- `fn match_mates(progenitors: &[T]) -> Vec<(T, T)>`: returns a pairing list of parent genotypes from a list of selected progenitors  

**Mutator module:** `mutator.rs`; provides the trait for mutating a genotype  
- Trait `Mutator`: generic over type `T`, which is the genotype type  
	- `fn mutate(individual: &mut T) -> ()`: mutates `individual` in-place  

**Recombinator module:** `recombinator.rs`; provides the trait for sexual (two-parent) crossover  
- Trait `Recombinator`: generic over type `T`, which is the genotype type  
	- `fn recombine(parent1: &T, parent2: &T) -> T`: returns a new genotype, which offspring was created through crossover of `parent1` and `parent2`  

**Reproducer module:** `reproducer.rs`; provides the trait for producing a new generation from a pairing of progenitor mates  
- Trait `Reproducer`: generic over types `T`, `Crossover`, and `Mutation`, which are the genotype type, `Recombinator`, and `Mutator` to use in reproduction  
	- `fn reproduce(mates: &[(T, T)], population_size: usize) -> Vec<T>`: returns the new offspring generation created using the `Crossover` and `Mutation` genetic operators applied to `mates`  

**Selector module:** `selector.rs`; provides the trait for selecting the list of progenitors from a population  
- Trait `Selector`: generic over types `T` and `Fitness`, which are the genotype type and `FitnessEvaluator` to use, respectively  
	- `fn select(population: &[T]) -> Vec<T>`: returns the progenitors selected from `population` according to the fitness scores assigned by `Fitness`  

#### Neural Net Crate

***This crate is not currently stable, thus details on its architecture or API are not provided at this time.***


## Contribution

Due to this project being in a very early R&D / WIP state, contributions are not being accepted at this time. Please feel free to fork and/or clone this repository as you wish, however!


## License

Copyright 2025 Jonathan Forsgren

This project is licensed under the [MIT license](https://github.com/Forjeon/conways-game-of-life-gacnn/blob/main/LICENSE)
