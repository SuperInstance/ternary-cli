/// Visualize subcommand: generate ASCII visualizations to terminal.
pub struct VisualizeCmd {
    pub vis_type: String,
    pub width: usize,
    pub height: usize,
}

impl VisualizeCmd {
    pub fn parse(args: &[&String]) -> Result<Self, String> {
        let mut vis_type = "fitness-landscape".to_string();
        let mut width = 60usize;
        let mut height = 20usize;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--type" | "-t" => {
                    i += 1;
                    vis_type = args.get(i)
                        .ok_or("--type requires a value")?
                        .to_string();
                }
                "--width" | "-w" => {
                    i += 1;
                    width = args.get(i)
                        .ok_or("--width requires a value")?
                        .parse::<usize>()
                        .map_err(|e| format!("invalid width: {}", e))?;
                }
                "--height" | "-h" => {
                    i += 1;
                    height = args.get(i)
                        .ok_or("--height requires a value")?
                        .parse::<usize>()
                        .map_err(|e| format!("invalid height: {}", e))?;
                }
                "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                other => return Err(format!("visualize: unknown option '{}'", other)),
            }
            i += 1;
        }

        if width < 10 {
            return Err("width must be >= 10".into());
        }
        if height < 5 {
            return Err("height must be >= 5".into());
        }

        Ok(VisualizeCmd { vis_type, width, height })
    }

    pub fn run(&self) -> Result<(), String> {
        println!("=== Ternary Visualization ===");
        println!("Type:   {}", self.vis_type);
        println!("Size:   {}x{}", self.width, self.height);
        println!();

        match self.vis_type.as_str() {
            "fitness-landscape" => self.draw_fitness_landscape(),
            "phase-space" => self.draw_phase_space(),
            "ternary-field" => self.draw_ternary_field(),
            other => return Err(format!("unknown visualization type: '{}'", other)),
        }
    }

    fn draw_fitness_landscape(&self) -> Result<(), String> {
        let w = self.width;
        let h = self.height;
        let mut grid = vec![vec![' '; w]; h];

        // Generate a fitness landscape: sin-based peaks
        for x in 0..w {
            let xf = x as f64 / w as f64 * std::f64::consts::PI * 2.0;
            let y_val = (xf * 2.0).sin() * 0.5 + 0.5;
            let y = h - 1 - (y_val * (h as f64 - 1.0)).round() as usize;
            let y = y.min(h - 1);

            // Draw the curve with shading
            for row in y..h {
                let depth = row - y;
                let ch = if depth == 0 { '█' } else if depth < 3 { '▓' } else if depth < 6 { '▒' } else { '░' };
                grid[row][x] = ch;
            }
        }

        // Add axis labels
        for row in &grid {
            let line: String = row.iter().collect();
            println!("{}", line);
        }

        println!();
        println!("  Fitness landscape (ternary strategy space)");
        Ok(())
    }

    fn draw_phase_space(&self) -> Result<(), String> {
        let w = self.width;
        let h = self.height;
        let mut grid = vec![vec![' '; w]; h];

        // Draw axes
        for x in 0..w { grid[h / 2][x] = '─'; }
        for y in 0..h { grid[y][w / 2] = '│'; }
        grid[h / 2][w / 2] = '┼';
        grid[0][w / 2] = '△';
        grid[h / 2][w - 1] = '▷';

        // Plot ternary points: -1 (defect), 0 (neutral), +1 (cooperate)
        let points = [
            (-0.8, 0.6), (0.7, 0.8), (0.0, -0.5),
            (-0.5, -0.3), (0.3, 0.1), (0.9, 0.0),
            (-0.2, 0.9), (0.5, -0.7), (0.0, 0.0),
        ];

        for (px, py) in &points {
            let x = (w as f64 / 2.0 + px * (w as f64 / 2.0 - 1.0)).round() as usize;
            let y = (h as f64 / 2.0 - py * (h as f64 / 2.0 - 1.0)).round() as usize;
            let x = x.min(w - 1);
            let y = y.min(h - 1);

            let ch = if *px > 0.3 && *py > 0.3 { '+' }
                     else if *px < -0.3 { '-' }
                     else { '·' };
            grid[y][x] = ch;
        }

        for row in &grid {
            let line: String = row.iter().collect();
            println!("{}", line);
        }

        println!();
        println!("  Phase space (+ cooperate, - defect, · neutral)");
        Ok(())
    }

    fn draw_ternary_field(&self) -> Result<(), String> {
        let w = self.width;
        let h = self.height;
        let chars = ['─', '·', '│']; // -1, 0, +1

        for y in 0..h {
            let mut line = String::with_capacity(w);
            for x in 0..w {
                // Deterministic ternary field pattern
                let val = ((x + y * 7) % 3) as usize;
                line.push(chars[val]);
            }
            println!("{}", line);
        }

        println!();
        println!("  Ternary field (─=-1 ·=0 │=+1)");
        Ok(())
    }
}

fn print_help() {
    let help = r#"ternary visualize — Generate ASCII visualizations

Usage:
  ternary visualize [options]

Options:
  --type, -t <TYPE>     Visualization type (default: fitness-landscape)
                        Types: fitness-landscape, phase-space, ternary-field
  --width, -w <N>       Width in characters (default: 60)
  --height, -h <N>      Height in rows (default: 20)
  --help                Show this help
"#;
    print!("{}", help);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_defaults() {
        let cmd = VisualizeCmd::parse(&[]).unwrap();
        assert_eq!(cmd.vis_type, "fitness-landscape");
        assert_eq!(cmd.width, 60);
        assert_eq!(cmd.height, 20);
    }

    #[test]
    fn parse_custom() {
        let args: Vec<String> = vec![
            "--type".into(), "phase-space".into(),
            "--width".into(), "80".into(),
            "--height".into(), "40".into(),
        ];
        let refs: Vec<&String> = args.iter().collect();
        let cmd = VisualizeCmd::parse(&refs).unwrap();
        assert_eq!(cmd.vis_type, "phase-space");
        assert_eq!(cmd.width, 80);
        assert_eq!(cmd.height, 40);
    }

    #[test]
    fn reject_small_width() {
        let args: Vec<String> = vec!["--width".into(), "5".into()];
        let refs: Vec<&String> = args.iter().collect();
        assert!(VisualizeCmd::parse(&refs).is_err());
    }

    #[test]
    fn run_fitness_landscape() {
        let cmd = VisualizeCmd {
            vis_type: "fitness-landscape".into(),
            width: 30,
            height: 10,
        };
        assert!(cmd.run().is_ok());
    }

    #[test]
    fn run_phase_space() {
        let cmd = VisualizeCmd {
            vis_type: "phase-space".into(),
            width: 30,
            height: 10,
        };
        assert!(cmd.run().is_ok());
    }

    #[test]
    fn run_ternary_field() {
        let cmd = VisualizeCmd {
            vis_type: "ternary-field".into(),
            width: 30,
            height: 10,
        };
        assert!(cmd.run().is_ok());
    }

    #[test]
    fn reject_unknown_type() {
        let cmd = VisualizeCmd {
            vis_type: "unknown".into(),
            width: 30,
            height: 10,
        };
        assert!(cmd.run().is_err());
    }
}
