use crate::{classify, benchmark, verify, visualize, evolve, config};

/// Parsed CLI arguments.
pub struct Cli {
    pub command: Command,
    pub config_path: Option<String>,
    pub show_version: bool,
    pub show_help: bool,
}

pub enum Command {
    Evolve(evolve::EvolveCmd),
    Classify(classify::ClassifyCmd),
    Benchmark(benchmark::BenchmarkCmd),
    Verify(verify::VerifyCmd),
    Visualize(visualize::VisualizeCmd),
    Help,
}

/// Parse command-line arguments into a structured Cli.
pub fn parse(args: &[String]) -> Result<Cli, String> {
    if args.is_empty() {
        return Err("no arguments provided".into());
    }

    let mut iter = args.iter();
    let _bin = iter.next(); // skip binary name

    let mut config_path: Option<String> = None;
    let mut show_version = false;
    let mut show_help = false;

    // Collect global flags first
    let mut positional: Vec<&String> = Vec::new();
    for arg in iter {
        if arg == "--version" || arg == "-V" {
            show_version = true;
        } else if arg == "--help" || arg == "-h" {
            show_help = true;
        } else if arg == "--config" {
            config_path = Some(String::new()); // placeholder, next arg fills it
        } else if config_path.as_deref() == Some("") {
            config_path = Some(arg.clone());
        } else {
            positional.push(arg);
        }
    }

    if show_version {
        return Ok(Cli {
            command: Command::Help,
            config_path: None,
            show_version: true,
            show_help: false,
        });
    }

    if show_help || positional.is_empty() {
        return Ok(Cli {
            command: Command::Help,
            config_path: None,
            show_version: false,
            show_help: true,
        });
    }

    let sub = positional[0];
    let sub_args = &positional[1..];

    let command = match sub.as_str() {
        "evolve" => Command::Evolve(evolve::EvolveCmd::parse(sub_args)?),
        "classify" => Command::Classify(classify::ClassifyCmd::parse(sub_args)?),
        "benchmark" => Command::Benchmark(benchmark::BenchmarkCmd::parse(sub_args)?),
        "verify" => Command::Verify(verify::VerifyCmd::parse(sub_args)?),
        "visualize" => Command::Visualize(visualize::VisualizeCmd::parse(sub_args)?),
        other => return Err(format!("unknown subcommand: '{}'", other)),
    };

    Ok(Cli {
        command,
        config_path,
        show_version: false,
        show_help: false,
    })
}

pub fn run(args: &[String]) -> Result<(), String> {
    let cli = parse(args)?;

    if cli.show_version {
        println!("ternary-cli 0.1.0");
        return Ok(());
    }

    if cli.show_help {
        print_help();
        return Ok(());
    }

    // Load config if available
    let _cfg = if let Some(ref path) = cli.config_path {
        Some(config::Config::load_from_file(path)?)
    } else {
        config::Config::load_default()
    };

    match cli.command {
        Command::Evolve(cmd) => cmd.run(),
        Command::Classify(cmd) => cmd.run(),
        Command::Benchmark(cmd) => cmd.run(),
        Command::Verify(cmd) => cmd.run(),
        Command::Visualize(cmd) => cmd.run(),
        Command::Help => {
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    let help = r#"ternary — CLI for the ternary agent ecosystem

Usage:
  ternary <command> [options]

Commands:
  evolve       Run evolution experiments
  classify     Classify strategies from a data file
  benchmark    Run benchmarks and display results
  verify       Verify conservation laws at multiple scales
  visualize    Generate ASCII visualizations

Global Options:
  --config <path>   Path to config file (default: ternary.toml)
  --version, -V     Print version
  --help, -h        Print this help message

Run `ternary <command> --help` for command-specific options.
"#;
    print!("{}", help);
}
