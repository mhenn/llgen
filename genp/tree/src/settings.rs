use config::{Config, ConfigError, File, FileFormat};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Population {
    pub mutation_rate: f32,
    pub empty_branch_rate: f32,
    pub tree_depth: usize,
    pub tree_width: usize,
}
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub population: Population,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::new("config/parameters", FileFormat::Ini))
            .build()?;

        // Now that we're done, let's access our configuration
        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

#[test]
fn config_note_empty() {
    let conf = Settings::new();
    assert!(conf.is_ok());
}
