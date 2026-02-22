use clap::{Arg, ArgAction, ArgMatches, Command};
use std::env;

use crate::constants::DEFAULT_CONFIG_PATH;

pub fn get_matches() -> ArgMatches {
  let directory_arg = Arg::new("directory")
    .short('d')
    .long("directory")
    .help("Relative path to the directory")
    .value_name("DIR");

  let config_arg = Arg::new("config")
    .short('c')
    .long("config")
    .global(true)
    .help("Path to the config file")
    .default_value(DEFAULT_CONFIG_PATH)
    .value_name("PATH");

  let storage_dir_arg = Arg::new("storage-dir")
    .short('s')
    .long("storage-dir")
    .global(true)
    .help("Path to the storage directory")
    .value_name("PATH");

  let full_path_arg = Arg::new("full-path")
    .long("full-path")
    .help("Full path to the file")
    .action(ArgAction::SetTrue);

  let porcelain_arg = Arg::new("porcelain")
    .long("porcelain")
    .help("Output in a machine-readable, structured format")
    .action(ArgAction::SetTrue);

  let filename_arg = Arg::new("filename")
    .help("Filename of the snippet")
    .value_name("FILENAME")
    .required(true);

  let list_command = Command::new("list")
    .about("List snippets")
    .arg(&directory_arg)
    .arg(&full_path_arg)
    .arg(&porcelain_arg);

  let create_command = Command::new("create")
    .about("Create a new snippet")
    .arg(&directory_arg)
    .arg(&filename_arg);

  let print_command = Command::new("print")
    .about("Print content of a snippet")
    .arg(&directory_arg)
    .arg(&filename_arg);

  let search_command = Command::new("search")
    .about("Search snippets")
    .arg(&directory_arg)
    .arg(&full_path_arg)
		.arg(&porcelain_arg)
    .arg(
      Arg::new("query")
        .help("Query to fuzzy search for")
        .value_name("QUERY")
        .required(true),
    );

  let delete_command = Command::new("delete")
    .about("Delete a snippet")
    .arg(&directory_arg)
    .arg(&filename_arg);

  Command::new("scribble")
    .version(env!("CARGO_PKG_VERSION"))
    .about("Manage snippets, notes, etc.")
    .arg_required_else_help(true)
    .arg(&config_arg)
    .arg(&storage_dir_arg)
    .subcommand(list_command)
    .subcommand(create_command)
    .subcommand(print_command)
    .subcommand(search_command)
    .subcommand(delete_command)
    .get_matches()
}
