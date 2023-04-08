mod steampunk_theme;
mod config;

use anyhow::Result;
use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use config::Config;
use steampunk_theme::steampunkify;

#[derive(Serialize, Deserialize)]
struct DataModel {
  name: String,
  path: PathBuf,
  // Additional metadata fields can be added here.
}

#[tokio::main]
async fn main() -> Result<()> {
  let matches = App::new("Library Manager")
    .about("Manage large data models in a steampunk-themed CLI")
    .author("Your Name <your.email@example.com>")
    .arg(
      Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("FILE")
        .help("Sets a custom config file")
        .takes_value(true),
    )
    .subcommand(
      SubCommand::with_name("show")
        .about("Show details of data models")
        .arg(
          Arg::with_name("MODEL")
            .help("The data model to display details for")
            .required(true)
            .index(1),
        ),
    )
    .subcommand(
      SubCommand::with_name("move")
        .about("Move a data model to a new location")
        .arg(
          Arg::with_name("MODEL")
            .help("The data model to move")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::with_name("DEST")
            .help("The new location for the data model")
            .required(true)
            .index(2),
        ),
    )
    .subcommand(
      SubCommand::with_name("delete")
        .about("Delete a data model")
        .arg(
          Arg::with_name("MODEL")
            .help("The data model to delete")
            .required(true)
            .index(1),
        ),
    )
    .get_matches();

  let config = Config::new(
      matches.value_of("config").unwrap_or("config.json"),
      "data_models.json",
  );
  let mut data_models: HashMap<String, DataModel> = load_data_models(&config)?;

  match matches.subcommand() {
    ("show", Some(matches)) => {
      let model_name = matches.value_of("MODEL").unwrap();
      show_model(&data_models, model_name)?;
    }
    ("move", Some(matches)) => {
      let model_name = matches.value_of("MODEL").unwrap();
      let dest = matches.value_of("DEST").unwrap();
      move_model(&mut data_models, model_name, dest).await?;
      save_data_models(&config, &data_models)?;
    }
    ("delete", Some(matches)) => {
      let model_name = matches.value_of("MODEL").unwrap();
      delete_model(&mut data_models, model_name).await?;
      save_data_models(&config, &data_models)?;
    }
    _ => {
      println!("Invalid command. Use --help for more information.");
    }
  }

  Ok(())
}

fn show_model(models: &HashMap<String, DataModel>, model_name: &str) -> Result<()> {
  if let Some(model) = models.get(model_name) {
    println!(
      "{}\nName: {}\nPath: {}",
      steampunkify("Model Details:"),
      model.name,
      model.path.display()
    );
  } else {
    println!("Model '{}' not found.", model_name);
  }

  Ok(())
}

async fn move_model(
  models: &mut HashMap<String, DataModel>,
  model_name: &str,
  dest: &str,
) -> Result<()> {
  if let Some(model) = models.get_mut(model_name) {
    let src = &model.path;
    let dest_path = PathBuf::from(dest);

    // Perform the actual move operation (this is a simple example, you may want to handle errors more gracefully)
    tokio::fs::rename(src, &dest_path).await?;

    // Update the model's path in the HashMap
    model.path = dest_path;

    println!("Moved '{}' to '{}'", model_name, dest);
  } else {
    println!("Model '{}' not found.", model_name);
  }

  Ok(())
}

async fn delete_model(models: &mut HashMap<String, DataModel>, model_name: &str) -> Result<()> {
  if let Some(model) = models.remove(model_name) {
    // Perform the actual delete operation (this is a simple example, you may want to handle errors more gracefully)
    tokio::fs::remove_dir_all(&model.path).await?;

    println!("Deleted '{}'", model_name);
  } else {
    println!("Model '{}' not found.", model_name);
  }

  Ok(())
}

fn load_data_models(config: &Config) -> Result<HashMap<String, DataModel>> {
  let mut file = fs::File::open(&config.config_file)
      .or_else(|_| fs::File::create(&config.config_file))?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let data_models: HashMap<String, DataModel> = if contents.is_empty() {
    HashMap::new()
  } else {
    serde_json::from_str(&contents)?
  };

  Ok(data_models)
}

fn save_data_models(config: &Config, data_models: &HashMap<String, DataModel>) -> Result<()> {
  let mut file = fs::File::create(&config.config_file)?;
  let contents = serde_json::to_string(data_models)?;
  file.write_all(contents.as_bytes())?;

  Ok(())
}
