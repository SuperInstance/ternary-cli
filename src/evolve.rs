/// Evolve subcommand: run evolution experiments with progress display.
pub struct EvolveCmd {
    pub generations: u64,
    pub population: usize,
    pub mutation_rate: f64,
    pub seed: Option<u64>,
}

impl EvolveCmd {
    pub fn parse(args: &[&String]) -> Result<Self, String> {
        let mut generations = 1000u64;
        let mut population = 100usize;
        let mut mutation_rate = 0.02f64;
        let mut seed: Option<u64> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--generations" | "-g" => {
                    i += 1;
                    generations = args.get(i)
                        .ok_or("--generations requires a value")?
                        .parse::<u64>()
                        .map_err(|e| format!("invalid generations: {}", e))?;
                }
                "--population" | "-p" => {
                    i += 1;
                    population = args.get(i)
                        .ok_or("--population requires a value")?
                        .parse::<usize>()
                        .map_err(|e| format!("invalid population: {}", e))?;
                }
                "--mutation-rate" | "-m" => {
                    i += 1;
                    mutation_rate = args.get(i)
                        .ok_or("--mutation-rate requires a value")?
                        .parse::<f64>()
                        .map_err(|e| format!("invalid mutation-rate: {}", e))?;
                }
                "--seed" | "-s" => {
                    i += 1;
                    seed = Some(args.get(i)
                        .ok_or("--seed requires a value")?
                        .parse::<u64>()
                        .map_err(|e| format!("invalid seed: {}", e))?);
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                other => return Err(format!("evolve: unknown option '{}'", other)),
            }
            i += 1;
        }

        if generations == 0 {
            return Err("generations must be > 0".into());
        }
        if population == 0 {
            return Err("population must be > 0".into());
        }
        if mutation_rate < 0.0 || mutation_rate > 1.0 {
            return Err("mutation-rate must be in [0, 1]".into());
        }

        Ok(EvolveCmd { generations, population, mutation_rate, seed })
    }

    pub fn run(&self) -> Result<(), String> {
        println!("=== Ternary Evolution Experiment ===");
        println!("Generations:     {}", self.generations);
        println!("Population:      {}", self.population);
        println!("Mutation rate:   {:.4}", self.mutation_rate);
        if let Some(s) = self.seed {
            println!("Seed:            {}", s);
        }
        println!();

        // Simple simulation: evolve ternary strategies (-1, 0, +1)
        let mut rng = SimpleRng::new(self.seed.unwrap_or(42));
        let mut best_fitness = 0.0f64;
        let mut best_gen = 0u64;

        // Initialize population
        let pop_size = self.population;
        let genome_len = 32;
        let mut population: Vec<Vec<i8>> = (0..pop_size)
            .map(|_| (0..genome_len).map(|_| (rng.next() as i8) % 3 - 1).collect())
            .collect();

        for gen in 1..=self.generations {
            // Evaluate fitness (sum of absolute values — favor diversity)
            let fitnesses: Vec<f64> = population.iter()
                .map(|genome| genome.iter().map(|&g| (g as f64).abs()).sum())
                .collect();

            // Track best
            let (_best_idx, &best_f) = fitnesses.iter().enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap();

            if best_f > best_fitness {
                best_fitness = best_f;
                best_gen = gen;
            }

            // Progress display
            if gen % (self.generations / 20).max(1) == 0 || gen == self.generations {
                let pct = gen as f64 / self.generations as f64;
                let bar = progress_bar(pct, 30);
                println!("[{}] Gen {}/{} | Best: {:.2} (gen {})", bar, gen, self.generations, best_fitness, best_gen);
            }

            // Selection + mutation for next generation
            let mut new_pop = Vec::with_capacity(pop_size);
            for _ in 0..pop_size {
                // Tournament selection
                let a = (rng.next() as usize) % pop_size;
                let b = (rng.next() as usize) % pop_size;
                let parent = if fitnesses[a] >= fitnesses[b] { a } else { b };
                let mut child = population[parent].clone();

                // Mutation
                for gene in &mut child {
                    let r = (rng.next() as f64) / u64::MAX as f64;
                    if r < self.mutation_rate {
                        *gene = (rng.next() as i8) % 3 - 1;
                    }
                }
                new_pop.push(child);
            }
            population = new_pop;
        }

        println!();
        println!("Evolution complete. Best fitness: {:.2} at generation {}", best_fitness, best_gen);
        Ok(())
    }
}

fn progress_bar(pct: f64, width: usize) -> String {
    let filled = (pct * width as f64).round() as usize;
    let empty = width - filled;
    format!("{}{}",
        "█".repeat(filled.min(width)),
        "░".repeat(empty),
    )
}

/// Minimal LCG-based RNG (no external deps).
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { state: if seed == 0 { 1 } else { seed } }
    }

    fn next(&mut self) -> u64 {
        // Numerical Recipes LCG
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }
}

fn print_help() {
    let help = r#"ternary evolve — Run evolution experiments

Usage:
  ternary evolve [options]

Options:
  --generations, -g <N>        Number of generations (default: 1000)
  --population, -p <N>         Population size (default: 100)
  --mutation-rate, -m <RATE>   Mutation rate 0..1 (default: 0.02)
  --seed, -s <N>               Random seed
  --help, -h                   Show this help
"#;
    print!("{}", help);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_defaults() {
        let cmd = EvolveCmd::parse(&[]).unwrap();
        assert_eq!(cmd.generations, 1000);
        assert_eq!(cmd.population, 100);
        assert!((cmd.mutation_rate - 0.02).abs() < 1e-9);
    }

    #[test]
    fn parse_custom() {
        let args: Vec<String> = vec![
            "--generations".into(), "500".into(),
            "--population".into(), "50".into(),
            "--mutation-rate".into(), "0.1".into(),
            "--seed".into(), "12345".into(),
        ];
        let refs: Vec<&String> = args.iter().collect();
        let cmd = EvolveCmd::parse(&refs).unwrap();
        assert_eq!(cmd.generations, 500);
        assert_eq!(cmd.population, 50);
        assert!((cmd.mutation_rate - 0.1).abs() < 1e-9);
        assert_eq!(cmd.seed, Some(12345));
    }

    #[test]
    fn reject_zero_generations() {
        let args: Vec<String> = vec!["--generations".into(), "0".into()];
        let refs: Vec<&String> = args.iter().collect();
        assert!(EvolveCmd::parse(&refs).is_err());
    }

    #[test]
    fn reject_bad_mutation_rate() {
        let args: Vec<String> = vec!["--mutation-rate".into(), "1.5".into()];
        let refs: Vec<&String> = args.iter().collect();
        assert!(EvolveCmd::parse(&refs).is_err());
    }

    #[test]
    fn run_short_experiment() {
        let cmd = EvolveCmd {
            generations: 10,
            population: 5,
            mutation_rate: 0.1,
            seed: Some(42),
        };
        assert!(cmd.run().is_ok());
    }
}
