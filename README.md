# ternary-cli

Command-line tools for the ternary computing ecosystem. Run evolution experiments, classify strategies, benchmark ternary vs binary, verify conservation laws, and generate ASCII visualizations — all from the terminal.

## Why This Exists

The ternary ecosystem spans 300+ crates. You need a Swiss army knife to interact with it without writing code. This CLI gives you hands-on access to Z₃ {-1, 0, +1} operations: evolve populations, classify data into ternary strategies, measure performance, and verify that conservation laws hold at scale.

The ah-ha moment: ternary operations have conservation properties that binary doesn't. In a balanced ternary system {-1, 0, +1}, summing all values over a complete cycle of 3 always gives zero. Energy is conserved. Charge is conserved. The `verify` command checks these properties at arbitrary scales — and they hold exactly, not approximately. No floating-point tolerance needed.

## Installation

```bash
cargo install ternary-cli
```

## Commands

### `ternary evolve` — Evolution Experiments

Run genetic algorithms where each organism's genome is a sequence of ternary values:

```bash
# Default: 1000 generations, population 100, 2% mutation rate
ternary evolve

# Custom experiment
ternary evolve --generations 5000 --population 200 --mutation-rate 0.05 --seed 42

# Watch it go:
# [████████████████████░░░░░░░░░░] Gen 350/5000 | Best: 30.00 (gen 280)
```

Each organism has a 32-gene ternary genome. Fitness is the sum of absolute values — favoring decisiveness (genes at ±1, not 0). Tournament selection picks parents, point mutations flip random genes. The built-in progress bar shows convergence.

Options:
- `--generations, -g <N>` — number of generations (default: 1000)
- `--population, -p <N>` — population size (default: 100)
- `--mutation-rate, -m <RATE>` — mutation probability (default: 0.02)
- `--seed, -s <N>` — random seed for reproducibility

### `ternary classify` — Strategy Classification

Classify data rows into ternary strategy types:

```bash
# Classify a CSV of numerical values
ternary classify data.csv

# TSV format
ternary classify data.tsv --format tsv
```

Each row of values is classified into one of six strategies:

| Strategy | Pattern | Detection Rule |
|----------|---------|----------------|
| **Cooperative** | Mostly positive | >70% values > 0 |
| **Defective** | Mostly negative | >70% values < 0 |
| **Ternary** | All three values present | >20% positive, >20% negative, >10% zero |
| **Tit-for-Tat** | Alternating ± | Low variance + balanced positive/negative |
| **Random** | High variance | Variance > 1.0 |
| **Unknown** | Doesn't fit above | Fallback |

Output includes per-row classification and a summary with percentages.

### `ternary benchmark` — Performance Measurement

Benchmark ternary arithmetic operations against each other:

```bash
# Run all benchmarks, 10K iterations each
ternary benchmark

# Custom iteration count and specific strategy
ternary benchmark --iterations 100000 --strategy add
```

Benchmarks:
- **Ternary add** — Z₃ addition across all input combinations
- **Ternary multiply** — sign multiplication
- **Ternary classify** — classify a ternary sequence
- **Conservation check** — verify sum properties at scale

Output:
```
Benchmark                  Iterations   Time (ms)   ns/iter
─────────────────────────────────────────────────────────────
Ternary add                   10000          1.2       120
Ternary multiply              10000          0.8        80
Ternary classify              10000          3.1       310
Conservation check            10000          2.4       240
```

### `ternary verify` — Conservation Law Verification

Verify that ternary conservation laws hold at multiple scales:

```bash
# Default: verify at scales 3, 5, 7, 9
ternary verify

# Custom scales and tolerance
ternary verify --scales 10,100,1000 --tolerance 1e-12
```

At each scale N, the verifier creates an N²-element ternary field and checks three conservation laws:
- **Energy** — sum of squared values
- **Mass** — sum of absolute values
- **Charge** — sum of values (should be exactly 0 over complete cycles)

```
Scale       Energy        Mass      Charge  Status
───────────────────────────────────────────────────────
3         2.00e+0     2.00e+0     0.00e+0  ✓ PASS
5         1.67e+1     1.67e+1     0.00e+0  ✓ PASS
7         3.27e+1     3.27e+1     0.00e+0  ✓ PASS
9         5.40e+1     5.40e+1     0.00e+0  ✓ PASS
```

### `ternary visualize` — ASCII Visualizations

Generate ASCII art representations of ternary grids and patterns:

```bash
ternary visualize
```

## Global Options

```
--config <path>   Path to config file (default: ternary.toml)
--version, -V     Print version
--help, -h        Print help
```

### Config File

Place a `ternary.toml` in your project directory:

```toml
[evolve]
generations = 1000
population = 100
mutation_rate = 0.02

[benchmark]
iterations = 10000

[verify]
scales = [3, 5, 7, 9]
tolerance = 1e-9
```

## Architecture

```
ternary-cli
├── src/main.rs       # Entry point — parse args, dispatch to subcommand
├── src/cli.rs        # Argument parser and help text
├── src/evolve.rs     # Evolution experiments with progress bar
├── src/classify.rs   # Strategy classification from data files
├── src/benchmark.rs  # Performance benchmarks with timing
├── src/verify.rs     # Conservation law verification at scale
├── src/visualize.rs  # ASCII visualization of ternary patterns
└── src/config.rs     # TOML config file loading
```

The CLI has zero external dependencies beyond `std`. The RNG for evolution experiments is a simple LCG (Numerical Recipes constants) — no `rand` crate needed.

## Design Decisions

**No clap/structopt**: The argument parser is hand-written (~100 lines). For a CLI with five subcommands and well-defined flags, the dependency cost of clap isn't worth it. The parser is easy to audit and modify.

**No rand crate**: Evolution experiments use a deterministic LCG. Given the same seed, you get the same result. This matters for reproducibility — if someone reports a bug, you can reproduce it exactly.

**No colored output**: The CLI uses plain ASCII characters (█░ for progress bars, ✓✗ for results). Works in every terminal, every pipe, every CI system.

## Ecosystem Connections

- **ternary-core** — Shared traits and Z₃ arithmetic that the CLI operations are based on
- **ternary-grid** — Spatial grid engine used by the verify command
- **ternary-automata** — Three-state cellular automata (future: visualize CA rules)
- **ternary-interpreter** — Bytecode VM (future: run ternary programs from the CLI)
- **ternary-compiler** — Expression compiler (future: compile and run expressions)

## Open Questions

- **CSV output**: Currently all output is human-readable text. Machine-readable CSV/JSON output would help scripting and benchmarking pipelines.
- **Pipe support**: The classify command reads from files, but stdin support would enable pipeline usage (`generate | classify | summarize`).
- **Parallel benchmarks**: Currently benchmarks run sequentially. Parallel execution would reduce wall-clock time on multi-core machines.

## Stats

| Metric | Value |
|--------|-------|
| LOC | ~1423 |
| Tests | 39 |
| Dependencies | 0 |
| Subcommands | 5 |

## License

Apache-2.0
