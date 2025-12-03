# Advent of Code

My solutions to https://adventofcode.com

## Usage

### Getting solution for a day

First, put the input for a problem in `inputs/YYYY/dXX.txt`, e.g. put day 1 from year 2024 in `inputs/2024/d01.txt`.

Then, run a specific day like this:

```bash
cargo run --release 2024 20  # release mode can be significantly faster
```

### Create skeleton code for new day

To create the skeleton code (copied from [template.rs](template.rs)), and update the year's `lib.rs` to import the new day's code and publish it's `Problem` implementation, run something like this:

```bash
cargo run new 2024 20
```

### Verify expected outputs

Verify that outputs match their expected values for all days like this:

```bash
cargo run --release verify
```

### Run unit tests

There's not many unit tests, but they can be run with

```bash
cargo test
# Or if you just want to run tests for one year
cargo test -p y2025
```

## File organization

```bash
├── cli          # CLI runner
├── crates
│   ├── shared   # Traits shared between all crates
│   ├── y2023    # Solutions for year 2023
│   ├── y2023    # Solutions for year 2024
│   └── y2025    # Solutions for year 2025
├── inputs
│   ├── 2023     # Put your 2023 inputs here
│   ├── 2024     # Put your 2024 inputs here
│   └── 2025     # Put your 2025 inputs here
└── template.rs  # Template file used as skeleton code for new day
```
