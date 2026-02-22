use crate::config::Config;
use crate::repositories::files::File;
use std::error::Error;

pub struct PrintFilesPathsArgs {
  pub files: Vec<File>,
  pub full_path: bool,
  pub porcelain: bool,
}

pub fn print_files_paths(args: PrintFilesPathsArgs) {
  for file in args.files {
    if args.porcelain {
      file.print_porcelain();
      continue;
    }

    if args.full_path {
      file.print_full_path();
      continue;
    }

    file.print_path();
  }
}

pub async fn create_file_in_sys(file_path: String) -> Result<(), Box<dyn Error>> {
  let path = std::path::Path::new(file_path.as_str());
  if !path.exists() {
    tokio::fs::create_dir_all(path.parent().unwrap()).await?;
    tokio::fs::File::create(path).await?;
  }

  Ok(())
}

pub async fn delete_file_in_sys(file_path: String) -> Result<(), Box<dyn Error>> {
  let path = std::path::Path::new(file_path.as_str());
  if path.exists() {
    tokio::fs::remove_file(path).await?;
  }

  Ok(())
}

pub async fn get_file_path(config: &Config, file: File) -> Result<String, Box<dyn Error>> {
  let path = file.path();
  let storage_path = config.storage_dir.clone();

  let full_path = format!("{}/{}", storage_path, path);
  Ok(shellexpand::tilde(&full_path).to_string())
}
