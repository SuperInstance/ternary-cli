/// Minimal TOML-like config parser (no external deps).
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Config {
    pub sections: HashMap<String, HashMap<String, String>>,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    /// Parse config from a string.
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut config = Config::new();
        let mut current_section = String::new();

        for (line_num, line) in input.lines().enumerate() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Section header [name]
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len() - 1].trim().to_string();
                if current_section.is_empty() {
                    return Err(format!("line {}: empty section name", line_num + 1));
                }
                config.sections.entry(current_section.clone()).or_default();
                continue;
            }

            // Key = value
            if let Some(eq_pos) = trimmed.find('=') {
                let key = trimmed[..eq_pos].trim().to_string();
                let value = trimmed[eq_pos + 1..].trim().to_string();

                if key.is_empty() {
                    return Err(format!("line {}: empty key", line_num + 1));
                }

                // Strip quotes from value
                let value = if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value[1..value.len() - 1].to_string()
                } else {
                    value
                };

                config.sections
                    .entry(current_section.clone())
                    .or_default()
                    .insert(key, value);
            } else {
                return Err(format!("line {}: expected 'key = value' or [section]", line_num + 1));
            }
        }

        Ok(config)
    }

    /// Load config from a file.
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("failed to read config '{}': {}", path, e))?;
        Self::parse(&content)
    }

    /// Try to load default config (ternary.toml in current directory).
    pub fn load_default() -> Option<Self> {
        std::fs::read_to_string("ternary.toml")
            .ok()
            .and_then(|content| Self::parse(&content).ok())
    }

    /// Get a value from the config.
    pub fn get(&self, section: &str, key: &str) -> Option<&str> {
        self.sections.get(section).and_then(|s| s.get(key)).map(|s| s.as_str())
    }

    /// Serialize config to string.
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        let mut sections: Vec<_> = self.sections.keys().collect();
        sections.sort();

        for section in sections {
            out.push_str(&format!("[{}]\n", section));
            let mut keys: Vec<_> = self.sections[section].keys().collect();
            keys.sort();
            for key in keys {
                out.push_str(&format!("{} = {}\n", key, self.sections[section][key]));
            }
            out.push('\n');
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let cfg = Config::parse("").unwrap();
        assert!(cfg.sections.is_empty());
    }

    #[test]
    fn parse_comments_and_blanks() {
        let input = "# comment\n\n  \n# another\n";
        let cfg = Config::parse(input).unwrap();
        assert!(cfg.sections.is_empty());
    }

    #[test]
    fn parse_section_with_values() {
        let input = "[evolve]\ngenerations = 1000\npopulation = 200\n";
        let cfg = Config::parse(input).unwrap();
        assert_eq!(cfg.get("evolve", "generations"), Some("1000"));
        assert_eq!(cfg.get("evolve", "population"), Some("200"));
    }

    #[test]
    fn parse_quoted_values() {
        let input = "[test]\nname = \"hello world\"\n";
        let cfg = Config::parse(input).unwrap();
        assert_eq!(cfg.get("test", "name"), Some("hello world"));
    }

    #[test]
    fn parse_single_quoted_values() {
        let input = "[test]\npath = '/some/path'\n";
        let cfg = Config::parse(input).unwrap();
        assert_eq!(cfg.get("test", "path"), Some("/some/path"));
    }

    #[test]
    fn reject_empty_section() {
        let input = "[]\n";
        assert!(Config::parse(input).is_err());
    }

    #[test]
    fn reject_no_equals() {
        let input = "bad line\n";
        assert!(Config::parse(input).is_err());
    }

    #[test]
    fn reject_empty_key() {
        let input = " = value\n";
        assert!(Config::parse(input).is_err());
    }

    #[test]
    fn serialize_roundtrip() {
        let input = "[evolve]\ngenerations = 1000\npopulation = 200\n";
        let cfg = Config::parse(input).unwrap();
        let output = cfg.to_string();
        assert!(output.contains("[evolve]"));
        assert!(output.contains("generations = 1000"));
        assert!(output.contains("population = 200"));
    }

    #[test]
    fn multiple_sections() {
        let input = "[evolve]\ngen = 100\n\n[benchmark]\niter = 50000\n";
        let cfg = Config::parse(input).unwrap();
        assert_eq!(cfg.get("evolve", "gen"), Some("100"));
        assert_eq!(cfg.get("benchmark", "iter"), Some("50000"));
    }
}
