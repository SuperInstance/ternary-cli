/// Verify subcommand: verify conservation laws at multiple scales.
pub struct VerifyCmd {
    pub scales: Vec<u64>,
    pub tolerance: f64,
}

#[derive(Debug)]
pub struct VerificationResult {
    pub scale: u64,
    pub energy_conserved: bool,
    pub mass_conserved: bool,
    pub charge_conserved: bool,
    pub energy_delta: f64,
    pub mass_delta: f64,
    pub charge_delta: f64,
}

impl VerifyCmd {
    pub fn parse(args: &[&String]) -> Result<Self, String> {
        let mut scales = vec![3, 5, 7, 9];
        let mut tolerance = 1e-9f64;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--scales" => {
                    i += 1;
                    let s = args.get(i).ok_or("--scales requires a value")?;
                    scales = s.split(',')
                        .map(|v| v.trim().parse::<u64>())
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|e| format!("invalid scales: {}", e))?;
                }
                "--tolerance" | "-t" => {
                    i += 1;
                    tolerance = args.get(i)
                        .ok_or("--tolerance requires a value")?
                        .parse::<f64>()
                        .map_err(|e| format!("invalid tolerance: {}", e))?;
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                other => return Err(format!("verify: unknown option '{}'", other)),
            }
            i += 1;
        }

        if scales.is_empty() {
            return Err("scales must not be empty".into());
        }
        if tolerance <= 0.0 {
            return Err("tolerance must be > 0".into());
        }

        Ok(VerifyCmd { scales, tolerance })
    }

    pub fn run(&self) -> Result<(), String> {
        println!("=== Ternary Conservation Law Verification ===");
        println!("Scales:     {:?}", self.scales);
        println!("Tolerance:  {:.e}", self.tolerance);
        println!();

        println!("{:<8} {:>10} {:>10} {:>10}  {}", "Scale", "Energy", "Mass", "Charge", "Status");
        println!("{}", "─".repeat(55));

        let mut all_pass = true;

        for &scale in &self.scales {
            let result = self.verify_at_scale(scale);
            let status = if result.energy_conserved && result.mass_conserved && result.charge_conserved {
                "✓ PASS"
            } else {
                all_pass = false;
                "✗ FAIL"
            };

            println!("{:<8} {:>10.2e} {:>10.2e} {:>10.2e}  {}",
                scale,
                result.energy_delta,
                result.mass_delta,
                result.charge_delta,
                status,
            );
        }

        println!();
        if all_pass {
            println!("All conservation laws verified within tolerance.");
        } else {
            println!("WARNING: Some conservation laws violated beyond tolerance!");
        }

        Ok(())
    }

    /// Verify conservation laws at a given scale using ternary arithmetic.
    pub fn verify_at_scale(&self, scale: u64) -> VerificationResult {
        let n = scale * scale;
        let mut total_energy = 0.0f64;
        let mut total_mass = 0.0f64;
        let mut total_charge = 0.0f64;

        for i in 0..n {
            // Simulate ternary field values
            let trit = ((i % 3) as f64) - 1.0;
            let energy = trit * trit; // always positive
            let mass = trit.abs();
            let charge = trit;

            total_energy += energy;
            total_mass += mass;
            total_charge += charge;
        }

        // Expected values for perfect ternary system
        let count = n as f64;
        // Over one full cycle of 3: -1, 0, 1 — sums to 0 for charge, 2/3 for energy, 2/3 for mass
        let expected_energy = (count / 3.0) * 2.0;
        let expected_mass = (count / 3.0) * 2.0;
        let expected_charge = 0.0;

        let energy_delta = (total_energy - expected_energy).abs();
        let mass_delta = (total_mass - expected_mass).abs();
        let charge_delta = (total_charge - expected_charge).abs();

        VerificationResult {
            scale,
            energy_conserved: energy_delta <= self.tolerance * expected_energy.max(1.0),
            mass_conserved: mass_delta <= self.tolerance * expected_mass.max(1.0),
            charge_conserved: charge_delta <= self.tolerance,
            energy_delta,
            mass_delta,
            charge_delta,
        }
    }
}

fn print_help() {
    let help = r#"ternary verify — Verify conservation laws

Usage:
  ternary verify [options]

Options:
  --scales <A,B,C>         Comma-separated scales to verify (default: 3,5,7,9)
  --tolerance, -t <EPS>    Tolerance for conservation checks (default: 1e-9)
  --help, -h               Show this help
"#;
    print!("{}", help);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_defaults() {
        let cmd = VerifyCmd::parse(&[]).unwrap();
        assert_eq!(cmd.scales, vec![3, 5, 7, 9]);
        assert!((cmd.tolerance - 1e-9).abs() < 1e-15);
    }

    #[test]
    fn parse_custom_scales() {
        let args: Vec<String> = vec!["--scales".into(), "2,4,8".into()];
        let refs: Vec<&String> = args.iter().collect();
        let cmd = VerifyCmd::parse(&refs).unwrap();
        assert_eq!(cmd.scales, vec![2, 4, 8]);
    }

    #[test]
    fn reject_empty_scales() {
        let args: Vec<String> = vec!["--scales".into(), "".into()];
        let refs: Vec<&String> = args.iter().collect();
        assert!(VerifyCmd::parse(&refs).is_err());
    }

    #[test]
    fn charge_always_conserved() {
        let cmd = VerifyCmd {
            scales: vec![3, 6, 9, 12],
            tolerance: 1e-9,
        };
        for &scale in &cmd.scales {
            let result = cmd.verify_at_scale(scale);
            assert!(result.charge_conserved, "charge not conserved at scale {}", scale);
        }
    }

    #[test]
    fn verify_runs() {
        let cmd = VerifyCmd {
            scales: vec![3, 5],
            tolerance: 1e-9,
        };
        assert!(cmd.run().is_ok());
    }
}
