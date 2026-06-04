/// Benchmark subcommand: run benchmarks and display results.
use std::time::Instant;

pub struct BenchmarkCmd {
    pub iterations: u64,
    pub strategy: String,
}

impl BenchmarkCmd {
    pub fn parse(args: &[&String]) -> Result<Self, String> {
        let mut iterations = 10000u64;
        let mut strategy = "all".to_string();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--iterations" | "-n" => {
                    i += 1;
                    iterations = args.get(i)
                        .ok_or("--iterations requires a value")?
                        .parse::<u64>()
                        .map_err(|e| format!("invalid iterations: {}", e))?;
                }
                "--strategy" | "-s" => {
                    i += 1;
                    strategy = args.get(i)
                        .ok_or("--strategy requires a value")?
                        .to_string();
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                other => return Err(format!("benchmark: unknown option '{}'", other)),
            }
            i += 1;
        }

        if iterations == 0 {
            return Err("iterations must be > 0".into());
        }

        Ok(BenchmarkCmd { iterations, strategy })
    }

    pub fn run(&self) -> Result<(), String> {
        println!("=== Ternary Benchmark Suite ===");
        println!("Iterations: {}", self.iterations);
        println!("Strategy:   {}", self.strategy);
        println!();

        let benchmarks = vec![
            ("Ternary add", bencher_add as fn(u64) -> u64),
            ("Ternary multiply", bencher_multiply as fn(u64) -> u64),
            ("Ternary classify", bencher_classify as fn(u64) -> u64),
            ("Conservation check", bencher_conservation as fn(u64) -> u64),
        ];

        println!("{:<25} {:>12} {:>12} {:>10}", "Benchmark", "Iterations", "Time (ms)", "ns/iter");
        println!("{}", "─".repeat(61));

        for (name, func) in &benchmarks {
            let start = Instant::now();
            let ops = func(self.iterations);
            let elapsed = start.elapsed();
            let ms = elapsed.as_millis();
            let ns_per = elapsed.as_nanos() as f64 / ops as f64;

            println!("{:<25} {:>12} {:>12} {:>10.1}", name, ops, ms, ns_per);
        }

        println!();
        println!("Benchmark complete.");
        Ok(())
    }
}

fn bencher_add(n: u64) -> u64 {
    let mut sum: i64 = 0;
    for i in 0..n {
        // Balanced ternary addition simulation
        let trit = ((i % 3) as i64) - 1;
        sum = sum.wrapping_add(trit);
    }
    // Prevent optimization
    if sum == i64::MAX { n } else { n }
}

fn bencher_multiply(n: u64) -> u64 {
    let mut product: i64 = 1;
    for i in 0..n {
        let trit = ((i % 3) as i64) - 1;
        if trit != 0 {
            product = product.wrapping_mul(trit.abs());
        }
    }
    if product == i64::MAX { n } else { n }
}

fn bencher_classify(n: u64) -> u64 {
    let data: Vec<f64> = (0..100).map(|i| ((i % 3) as f64) - 1.0).collect();
    let mut count = 0u64;
    for _ in 0..n {
        let _ = crate::classify::classify_values(&data);
        count += 1;
    }
    count
}

fn bencher_conservation(n: u64) -> u64 {
    let values: Vec<f64> = (0..50).map(|i| ((i % 3) as f64) - 1.0).collect();
    let mut count = 0u64;
    for _ in 0..n {
        let sum: f64 = values.iter().sum();
        let _ = sum.abs() < 100.0;
        count += 1;
    }
    count
}

fn print_help() {
    let help = r#"ternary benchmark — Run benchmarks

Usage:
  ternary benchmark [options]

Options:
  --iterations, -n <N>   Number of iterations (default: 10000)
  --strategy, -s <NAME>  Strategy to benchmark: all, add, mul (default: all)
  --help, -h             Show this help
"#;
    print!("{}", help);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_defaults() {
        let cmd = BenchmarkCmd::parse(&[]).unwrap();
        assert_eq!(cmd.iterations, 10000);
        assert_eq!(cmd.strategy, "all");
    }

    #[test]
    fn parse_custom() {
        let args: Vec<String> = vec![
            "--iterations".into(), "5000".into(),
            "--strategy".into(), "add".into(),
        ];
        let refs: Vec<&String> = args.iter().collect();
        let cmd = BenchmarkCmd::parse(&refs).unwrap();
        assert_eq!(cmd.iterations, 5000);
        assert_eq!(cmd.strategy, "add");
    }

    #[test]
    fn reject_zero_iterations() {
        let args: Vec<String> = vec!["--iterations".into(), "0".into()];
        let refs: Vec<&String> = args.iter().collect();
        assert!(BenchmarkCmd::parse(&refs).is_err());
    }

    #[test]
    fn run_benchmark() {
        let cmd = BenchmarkCmd {
            iterations: 100,
            strategy: "all".into(),
        };
        assert!(cmd.run().is_ok());
    }
}
