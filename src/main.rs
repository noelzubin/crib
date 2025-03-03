pub use app::App;

pub mod app;

mod config_parser;

use structopt::StructOpt;

const DEFAULT_YAML_CONFIG: &str = r#"
- name: app
  children:
    - name: subcategory
      bindings:
        - name: do something
          key: ctrl + a
        - name: do something else
          key: ctrl + b
  bindings: 
  - name: one 
    key: shift + c
- name: tmux
  bindings: 
  - name: split vertical
    key: ctrl + h
  - name: split horizontal
    key: ctrl + l
"#;

/// A simple CLI tool
#[derive(StructOpt, Debug)]
#[structopt(name = "crib")]
struct Opt {
    /// The initial query
    query: Option<String>,
    /// Print the configuration
    #[structopt(long)]
    print: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    use dirs::home_dir;
    use std::fs;

    let opt = Opt::from_args();

    let initial_query = match opt.query {
        Some(query) => format!("{}:", query),
        None => String::new(),
    };

    let config_path = home_dir()
        .map(|p| p.join(".config/crib/bindings.yaml"))
        .expect("Failed to determine home directory");

    if !config_path.exists() {
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(&config_path, DEFAULT_YAML_CONFIG)?;
    }

    let config = config_parser::parse_config(config_path.to_str().unwrap()).unwrap();
    let table_configs = config.get_table_configs();
    let app = App::new(initial_query, table_configs);

    let result = if opt.print {
        app.draw_to_buffer();
        Ok(())
    } else {
        let terminal = ratatui::init();
        let result = app.run(terminal);
        ratatui::restore();
        result
    };
    result
}
