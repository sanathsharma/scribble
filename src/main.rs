#![allow(dead_code)]

mod args;
mod config;
mod constants;
mod repositories;
mod utils;

use repositories::files::FromMatches;
use std::{error::Error, process::Stdio};
use tokio::process::Command;

use crate::{
  repositories::files::{
    CreateArgs, DeleteArgs, FileRepository, GetFileArgs, ListArgs, SearchArgs,
  },
  utils::{
    db::create_db_pool,
    path::{
      PrintFilesPathsArgs, create_file_in_sys, delete_file_in_sys, get_file_path, print_files_paths,
    },
  },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let matches = args::get_matches();
  let pool = create_db_pool().await?;
  let file_repo = FileRepository::new(pool);

  let config = config::get_config(&matches).await;

  match matches.subcommand() {
    Some(("list", args)) => {
      let list_args = ListArgs::from_matches(args).await?;
      let files = file_repo.list(list_args).await?;

      print_files_paths(PrintFilesPathsArgs {
        files,
        full_path: args.get_flag("full-path"),
        porcelain: args.get_flag("porcelain"),
      });
    }
    Some(("create", args)) => {
      let args = CreateArgs::from_matches(args).await?;
      let file = file_repo.create(args).await?;
      let file_path = get_file_path(config, file).await?;
      create_file_in_sys(file_path).await?;

      println!("File created successfully!");
    }
    Some(("print", args)) => {
      let args = GetFileArgs::from_matches(args).await?;
      let file = file_repo.get_file(args).await?;

      Command::new("cat")
        .arg(file.path())
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status()
        .await?;
    }
    Some(("search", args)) => {
      let search_args = SearchArgs::from_matches(args).await?;
      let files = file_repo.search(search_args).await?;

      print_files_paths(PrintFilesPathsArgs {
        files,
        full_path: args.get_flag("full-path"),
        porcelain: args.get_flag("porcelain"),
      });
    }
    Some(("delete", args)) => {
      let args = DeleteArgs::from_matches(args).await?;
      let file = file_repo.delete(args).await?;
      let file_path = get_file_path(config, file).await?;
      delete_file_in_sys(file_path).await?;

      println!("File deleted successfully!");
    }
    _ => {}
  }

  Ok(())
}
