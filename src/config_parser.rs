use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mapping {
    name: String,
    children: Option<Vec<Mapping>>,
    bindings: Vec<Binding>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Binding {
    pub name: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config(Vec<Mapping>);

pub fn parse_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[derive(Debug)]
pub struct TableConfig {
    pub name: String,
    pub bindings: Vec<Binding>,
}

fn add_mapping_to_list(prefix:&str, tables: &mut Vec<TableConfig>, mapping: &Mapping) {
    let mapping_name = if prefix.is_empty() { mapping.name.to_string() } else { format!("{}:{}", prefix , &mapping.name) };
    let table_config = TableConfig{
        name: mapping_name.clone(),
        bindings: mapping.bindings.clone(),
    };
    tables.push(table_config);

    if let Some(children) = &mapping.children {
        for child in children {
            add_mapping_to_list(&mapping_name, tables, child);
        }
    }
}

impl Config {
    pub fn get_table_configs(&self) -> Vec<TableConfig> {
        let mut table_configs = Vec::new();

        for mapping in self.0.iter() {
            add_mapping_to_list("", &mut table_configs, &mapping);
        }

        table_configs
    }
}
