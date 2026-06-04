# ternary-cli

Command-line interface for the **ternary agent ecosystem** — a terminal tool for running experiments, visualizing results, and exploring ternary agent theory.

## Installation

```bash
cargo install --path .
```

## Usage

### Evolve

Run an evolution experiment with progress display:

```bash
ternary evolve --generations 500 --population 100
ternary evolve -g 200 -p 50 --mutation-rate 0.05
```

### Classify

Classify strategies from a data file:

```bash
ternary classify data.csv
ternary classify input.txt --format csv
```

### Benchmark

Run benchmarks and display results:

```bash
ternary benchmark
ternary benchmark --iterations 10000 --strategy all
```

### Verify

Verify conservation laws at multiple scales:

```bash
ternary verify
ternary verify --scales 3,5,7 --tolerance 1e-9
```

### Visualize

Generate ASCII visualizations in the terminal:

```bash
ternary visualize
ternary visualize --type fitness-landscape --width 80
ternary visualize --type phase-space --height 40
```

### Global Options

```bash
ternary --help
ternary --version
ternary --config custom.toml evolve
```

## Configuration

Create a `ternary.toml` file in the current directory or pass `--config <path>`:

```toml
[evolve]
generations = 1000
population = 200
mutation_rate = 0.02

[benchmark]
iterations = 50000

[visualize]
width = 80
height = 24
```

## License

MIT
