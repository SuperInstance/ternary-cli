/// Classify subcommand: classify strategies from a data file.
pub struct ClassifyCmd {
    pub input_file: String,
    pub format: String,
}

#[derive(Debug, PartialEq)]
pub enum Strategy {
    Cooperative,
    Defective,
    TitForTat,
    Random,
    Ternary,
    Unknown(String),
}

impl std::fmt::Display for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Strategy::Cooperative => write!(f, "Cooperative"),
            Strategy::Defective => write!(f, "Defective"),
            Strategy::TitForTat => write!(f, "Tit-for-Tat"),
            Strategy::Random => write!(f, "Random"),
            Strategy::Ternary => write!(f, "Ternary"),
            Strategy::Unknown(s) => write!(f, "Unknown({})", s),
        }
    }
}

impl ClassifyCmd {
    pub fn parse(args: &[&String]) -> Result<Self, String> {
        let mut input_file: Option<String> = None;
        let mut format = "csv".to_string();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--format" | "-f" => {
                    i += 1;
                    format = args.get(i)
                        .ok_or("--format requires a value")?
                        .to_string();
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                other if !other.starts_with('-') && input_file.is_none() => {
                    input_file = Some(other.to_string());
                }
                other => return Err(format!("classify: unknown option '{}'", other)),
            }
            i += 1;
        }

        let input_file = input_file.ok_or("classify: input file required")?;

        Ok(ClassifyCmd { input_file, format })
    }

    pub fn run(&self) -> Result<(), String> {
        println!("=== Ternary Strategy Classification ===");
        println!("Input:  {}", self.input_file);
        println!("Format: {}", self.format);

        let content = std::fs::read_to_string(&self.input_file)
            .map_err(|e| format!("failed to read '{}': {}", self.input_file, e))?;

        let rows: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
        println!("Rows:   {}", rows.len());
        println!();

        let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (i, row) in rows.iter().enumerate() {
            let values = parse_row(row, &self.format);
            let strategy = classify_values(&values);
            *counts.entry(strategy.to_string()).or_insert(0) += 1;

            if i < 10 {
                println!("  Row {:>3}: {:?} → {}", i + 1, truncate(&values, 8), strategy);
            }
        }

        if rows.len() > 10 {
            println!("  ... ({} more rows)", rows.len() - 10);
        }

        println!();
        println!("Classification summary:");
        for (strategy, count) in &counts {
            let pct = *count as f64 / rows.len() as f64 * 100.0;
            println!("  {:>15}: {:>4} ({:.1}%)", strategy, count, pct);
        }

        Ok(())
    }
}

fn parse_row(row: &str, _format: &str) -> Vec<f64> {
    row.split(|c: char| c == ',' || c.is_whitespace())
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect()
}

fn truncate(v: &[f64], max: usize) -> Vec<f64> {
    if v.len() <= max {
        v.to_vec()
    } else {
        let mut t = v[..max].to_vec();
        t.push(f64::NAN);
        t
    }
}

/// Classify a sequence of values into a strategy type.
pub fn classify_values(values: &[f64]) -> Strategy {
    if values.is_empty() {
        return Strategy::Unknown("empty".into());
    }

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter()
        .map(|&v| (v - mean).powi(2))
        .sum::<f64>() / values.len() as f64;

    let positive = values.iter().filter(|&&v| v > 0.0).count() as f64 / values.len() as f64;
    let negative = values.iter().filter(|&&v| v < 0.0).count() as f64 / values.len() as f64;
    let zero = values.iter().filter(|&&v| v == 0.0).count() as f64 / values.len() as f64;

    // Ternary: significant proportion of all three categories (-1, 0, +1)
    if positive > 0.2 && negative > 0.2 && zero > 0.1 {
        return Strategy::Ternary;
    }

    // Cooperative: mostly positive
    if positive > 0.7 {
        return Strategy::Cooperative;
    }

    // Defective: mostly negative
    if negative > 0.7 {
        return Strategy::Defective;
    }

    // Tit-for-tat: alternating pattern with low variance
    if variance < 0.5 && positive > 0.3 && negative > 0.3 {
        return Strategy::TitForTat;
    }

    // Random: high variance
    if variance > 1.0 {
        return Strategy::Random;
    }

    Strategy::Unknown("mixed".into())
}

fn print_help() {
    let help = r#"ternary classify — Classify strategies from a data file

Usage:
  ternary classify <file> [options]

Options:
  --format, -f <FORMAT>   Input format: csv, tsv (default: csv)
  --help, -h              Show this help
"#;
    print!("{}", help);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_cooperative() {
        let vals = vec![1.0, 1.0, 1.0, 1.0, 0.8];
        assert_eq!(classify_values(&vals), Strategy::Cooperative);
    }

    #[test]
    fn classify_defective() {
        let vals = vec![-1.0, -1.0, -1.0, -0.9, -1.0];
        assert_eq!(classify_values(&vals), Strategy::Defective);
    }

    #[test]
    fn classify_ternary() {
        let vals = vec![1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0];
        assert_eq!(classify_values(&vals), Strategy::Ternary);
    }

    #[test]
    fn classify_random() {
        let vals = vec![10.0, -10.0, 5.0, -5.0];
        assert_eq!(classify_values(&vals), Strategy::Random);
    }

    #[test]
    fn classify_empty() {
        assert_eq!(classify_values(&[]), Strategy::Unknown("empty".into()));
    }

    #[test]
    fn parse_requires_file() {
        let result = ClassifyCmd::parse(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn parse_row_csv() {
        let row = "1.0, -1.0, 0.0, 0.5";
        let vals = parse_row(row, "csv");
        assert_eq!(vals, vec![1.0, -1.0, 0.0, 0.5]);
    }

    #[test]
    fn run_classify_file_not_found() {
        let cmd = ClassifyCmd {
            input_file: "/nonexistent/file.csv".into(),
            format: "csv".into(),
        };
        assert!(cmd.run().is_err());
    }
}
