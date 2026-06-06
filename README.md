# ternary-cli

CLI tools for the ternary computing ecosystem. Inspect, validate, and benchmark Z₃ {-1, 0, +1} operations from the command line.

## Why This Matters

The ternary ecosystem spans 300+ crates covering everything from neural networks to distributed consensus. This CLI gives you hands-on access to Z₃ operations — inspect ternary states, validate conservation laws, and benchmark ternary vs binary performance.

## The Five-Layer Stack

```
┌─────────────────┐
│  cudaclaw        │  Persistent GPU kernels, warp consensus, SmartCRDT
├─────────────────┤
│  cuda-oxide      │  Flux → MIR → Pliron → NVVM → PTX compiler
├─────────────────┤
│  flux-core       │  Bytecode VM + A2A agent protocol
├─────────────────┤
│  pincher         │  "Vector DB as runtime, LLM as compiler"
├─────────────────┤
│  open-parallel   │  Async runtime (tokio fork)
└─────────────────┘
```

## Installation

```bash
cargo install ternary-cli
```

## Usage

```bash
# Inspect a ternary value
ternary inspect --value 1
ternary inspect --value -1
ternary inspect --value 0

# Validate conservation
ternary validate --check conservation

# Benchmark ternary vs binary
ternary benchmark --operations 10000
```

## Stats

| Metric | Value |
|--------|-------|
| LOC | 1423 |
| Tests | 39 |

## License

Apache-2.0
