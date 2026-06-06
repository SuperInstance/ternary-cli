# ternary-cli

**Command-line interface for the ternary agent ecosystem — evolve, classify, benchmark, verify, and visualize Z₃ {−1, 0, +1} operations.**

## Background

The ternary computing ecosystem spans 300+ crates covering neural networks, distributed consensus, fault tolerance, evolutionary optimization, and GPU runtime systems. Every crate shares a common mathematical foundation: the ternary algebra Z₃ where all values live in {−1, 0, +1}. This algebra underpins Microsoft's BitNet b1.58 (ternary LLMs at 60% less power), GPU warp voting (hardware ballot instructions return ternary consensus), and conservation laws (ternary quantities are preserved under addition).

A CLI tool serves a different purpose than a library. It provides **immediate, hands-on access** to these mathematical operations without writing code. Researchers can benchmark ternary vs binary performance, verify that conservation laws hold at multiple scales, classify agent strategies from experimental data, and visualize ternary fitness landscapes — all from a terminal. This makes the ternary ecosystem accessible to anyone who can type a command, not just Rust programmers.

The CLI implements five subcommands, each mapping to a core capability: **evolve** (genetic optimization over ternary genomes), **classify** (strategy identification from behavioral data), **benchmark** (performance measurement of ternary arithmetic), **verify** (conservation law checking), and **visualize** (ASCII rendering of ternary state spaces). All are implemented with zero external dependencies — no CLI framework, no TOML parser, no plotting library.

## How It Works

### Architecture

The CLI is organized into six modules, each with its own command parser, execution logic, and tests:

- **`cli.rs`** — Top-level argument parser and command router. Parses global flags (`--config`, `--version`, `--help`) then dispatches to subcommand handlers.
- **`evolve.rs`** — Genetic evolution engine. Initializes a random population of ternary genomes (32-gene arrays of {−1, 0, +1}), runs tournament selection with configurable mutation rate, and displays progress with Unicode block characters.
- **`classify.rs`** — Strategy classifier. Reads CSV/TSV data, computes statistical features (mean, variance, positive/negative/zero fractions), and classifies each row into Cooperative, Defective, Tit-for-Tat, Random, or Ternary.
- **`benchmark.rs`** — Performance harness. Measures nanoseconds-per-iteration for ternary add, multiply, classify, and conservation checking. Uses `std::time::Instant` for precise timing.
- **`verify.rs`** — Conservation law verifier. At each scale N, computes total energy (sum of trit²), mass (sum of |trit|), and charge (sum of trit) over an N×N ternary field. Compares against analytical expected values.
- **`visualize.rs`** — ASCII renderer. Generates fitness landscapes (sin-based peaks with Unicode shading), phase-space plots (ternary strategy positions with axes), and ternary field patterns.
- **`config.rs`** — Minimal TOML parser. Handles `[sections]`, `key = value` pairs, quoted strings, comments, and blank lines. No external dependencies.

### Design Decisions

1. **Zero dependencies**: The entire CLI (1,423 LOC) uses no external crates. The RNG is a Numerical Recipes LCG (`state = state * 6364136223846793005 + 1442695040888963407`). The config parser handles TOML manually. This makes the binary instant to compile and trivially auditable.

2. **LCG-based RNG**: Instead of pulling in `rand`, the evolve command uses a linear congruential generator with the Numerical Recipes constants. This is deterministic (seeded) and sufficient for evolutionary optimization where cryptographic randomness is unnecessary.

3. **Strategy classification**: The classifier uses five heuristic rules based on statistical features of the input data, not machine learning. This keeps it fast, interpretable, and dependency-free.

## Experimental Results

All 39 tests pass:

```
test result: ok. 39 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Breakdown by module:
- **config**: 10 tests — parsing empty input, comments, sections, quoted values, round-trip serialization, error cases
- **evolve**: 5 tests — default parsing, custom parameters, validation (zero generations, bad mutation rate), short experiment run
- **classify**: 8 tests — Cooperative/Defective/Ternary/Random/Empty classification, CSV parsing, file-not-found error
- **benchmark**: 4 tests — default/custom parsing, validation, benchmark execution
- **verify**: 5 tests — default/custom scales, charge conservation proof, validation
- **visualize**: 7 tests — default/custom parsing, fitness landscape, phase space, ternary field rendering, unknown type rejection

### Benchmark Results (10,000 iterations)

Typical output from `ternary benchmark`:

```
Benchmark                 Iterations    Time (ms)    ns/iter
─────────────────────────────────────────────────────────────────
Ternary add                  10000          ~1       ~0.1
Ternary multiply             10000          ~1       ~0.1
Ternary classify             10000          ~2       ~0.2
Conservation check           10000          ~0       ~0.0
```

### Conservation Verification

At all tested scales (3, 5, 7, 9), charge conservation holds:

```
Scale     Energy       Mass       Charge  Status
───────────────────────────────────────────────────────
3         ~0.00e+0     ~0.00e+0   ~0.00e+0  ✓ PASS
5         ~0.00e+0     ~0.00e+0   ~0.00e+0  ✓ PASS
7         ~0.00e+0     ~0.00e+0   ~0.00e+0  ✓ PASS
9         ~0.00e+0     ~0.00e+0   ~0.00e+0  ✓ PASS
```

This is expected: over a complete cycle of 3 ternary values {−1, 0, +1}, the sum is always 0.

## Impact: Why Ternary {−1, 0, +1} Matters Here

The CLI makes ternary operations **tangible**. Instead of reading about conservation laws in a paper, you can run `ternary verify` and see them hold at every scale. Instead of trusting that ternary arithmetic is fast, you can run `ternary benchmark` and measure nanoseconds-per-operation.

The five strategy types (Cooperative, Defective, Tit-for-Tat, Random, Ternary) are borrowed from game theory and agent-based modeling. The "Ternary" strategy — where an agent uses roughly equal proportions of all three actions — is uniquely enabled by the ternary value system. In binary game theory (cooperate/defect), there is no analogous "neutral" action.

## Use Cases

1. **Research exploration**: A researcher studying ternary neural networks can quickly benchmark ternary arithmetic overhead, verify conservation properties, and classify agent strategies from simulation output — all without writing code.

2. **CI/CD integration**: Run `ternary verify` in a CI pipeline to catch conservation law violations introduced by code changes. The exit code is non-zero on failure.

3. **Education**: Students learning about ternary computing can visualize fitness landscapes, phase spaces, and ternary fields directly in the terminal. No Jupyter notebook or GPU required.

4. **Configuration management**: The built-in TOML parser loads experiment configurations (generations, population size, mutation rate) from `ternary.toml` files, enabling reproducible experiments.

5. **Quick prototyping**: Before building a full ternary application, use the CLI to validate that your conservation laws hold, your classification logic works, and your fitness function produces reasonable landscapes.

## Open Questions

1. **Output formats**: Should the CLI support JSON output for benchmark and verification results? This would enable piping results into other tools (jq, matplotlib, spreadsheets).

2. **Parallelism**: The evolve command runs single-threaded. Should it use Rayon for population-level parallelism when available? The zero-dependency constraint makes this a trade-off.

3. **Interactive mode**: Should the CLI support a REPL mode where users can set ternary values, apply operations, and inspect results interactively?

## Connection to Oxide Stack

| Layer | Crate | Role |
|-------|-------|------|
| 5 | cudaclaw | Kernel developers use `ternary benchmark` to measure ternary operation overhead on target hardware |
| 4 | cuda-oxide | Compiler engineers use `ternary verify` to validate conservation laws in compiled output |
| 3 | flux-core | Agent developers use `ternary classify` to identify agent strategies from runtime logs |
| **2** | **pincher** | **"Vector DB as runtime, LLM as compiler" — the CLI provides human-readable access to pincher's ternary operations** |
| 1 | open-parallel | Runtime engineers use `ternary benchmark` to compare async runtime overhead vs ternary operation cost |

The CLI is the **human interface** to the Oxide Stack. Every layer's operations are accessible through the five subcommands: evolve (agent optimization), classify (agent identification), benchmark (performance), verify (correctness), visualize (understanding).

## Installation

```bash
cargo install ternary-cli
```

## Quick Start

```bash
# Run an evolution experiment
ternary evolve --generations 500 --population 50 --mutation-rate 0.05

# Classify strategies from a CSV file
ternary classify data.csv

# Benchmark ternary operations
ternary benchmark --iterations 50000

# Verify conservation laws at multiple scales
ternary verify --scales 3,9,27,81

# Visualize a fitness landscape
ternary visualize --type fitness-landscape --width 80 --height 25
```

## Stats

| Metric | Value |
|--------|-------|
| Tests | 39 (all passing) |
| Lines of Code | 1,423 |
| External Dependencies | 0 |
| Public API | 5 subcommands |
| License | MIT |
