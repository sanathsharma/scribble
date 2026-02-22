use crate::config;
use async_trait::async_trait;

#[derive(sqlx::FromRow)]
pub struct File {
  pub id: i64,
  pub directory: String,
  pub filename: String,
  pub storage_dir: String,
}

pub struct FileRepository {
  pool: sqlx::SqlitePool,
}

#[derive(Debug)]
pub struct ListArgs {
  pub directory: Option<String>,
  pub storage_dir: String,
}

#[derive(Debug)]
pub struct CreateArgs {
  pub directory: Option<String>,
  pub filename: String,
  pub storage_dir: String,
}

#[derive(Debug)]
pub struct GetFileArgs {
  pub directory: Option<String>,
  pub filename: String,
  pub storage_dir: String,
}

#[derive(Debug)]
pub struct SearchArgs {
  pub directory: Option<String>,
  pub query: String,
  pub storage_dir: String,
}

#[derive(Debug)]
pub struct DeleteArgs {
  pub directory: Option<String>,
  pub filename: String,
  pub storage_dir: String,
}

#[async_trait]
pub trait FromMatches {
  async fn from_matches(matches: &clap::ArgMatches) -> Result<Self, String>
  where
    Self: Sized;
}

#[async_trait]
impl FromMatches for ListArgs {
  async fn from_matches(args: &clap::ArgMatches) -> Result<Self, String> {
    Ok(Self {
      directory: args
        .try_get_one::<String>("directory")
        .unwrap()
        .map(|s| s.to_string()),
      storage_dir: get_storage_dir(args).await,
    })
  }
}

#[async_trait]
impl FromMatches for CreateArgs {
  async fn from_matches(args: &clap::ArgMatches) -> Result<Self, String> {
    Ok(Self {
      directory: args
        .try_get_one::<String>("directory")
        .unwrap()
        .map(|s| s.to_string()),
      filename: args.get_one::<String>("filename").unwrap().to_string(),
      storage_dir: get_storage_dir(args).await,
    })
  }
}

#[async_trait]
impl FromMatches for GetFileArgs {
  async fn from_matches(args: &clap::ArgMatches) -> Result<Self, String> {
    Ok(Self {
      directory: args
        .try_get_one::<String>("directory")
        .unwrap()
        .map(|s| s.to_string()),
      filename: args.get_one::<String>("filename").unwrap().to_string(),
      storage_dir: get_storage_dir(args).await,
    })
  }
}

#[async_trait]
impl FromMatches for SearchArgs {
  async fn from_matches(args: &clap::ArgMatches) -> Result<Self, String> {
    Ok(Self {
      directory: args
        .try_get_one::<String>("directory")
        .unwrap()
        .map(|s| s.to_string()),
      query: args.get_one::<String>("query").unwrap().to_string(),
      storage_dir: get_storage_dir(args).await,
    })
  }
}

#[async_trait]
impl FromMatches for DeleteArgs {
  async fn from_matches(args: &clap::ArgMatches) -> Result<Self, String> {
    Ok(Self {
      directory: args
        .try_get_one::<String>("directory")
        .unwrap()
        .map(|s| s.to_string()),
      filename: args.get_one::<String>("filename").unwrap().to_string(),
      storage_dir: get_storage_dir(args).await,
    })
  }
}

impl FileRepository {
  pub fn new(pool: sqlx::SqlitePool) -> Self {
    Self { pool }
  }

  pub async fn list(&self, args: ListArgs) -> Result<Vec<File>, String> {
    let mut builder = sqlx::QueryBuilder::new("SELECT * FROM files WHERE storage_dir = ");
    builder.push_bind(args.storage_dir);

    if let Some(directory) = args.directory {
      builder.push(" AND directory = ").push_bind(directory);
    }

    let files = builder
      .build_query_as::<File>()
      .fetch_all(&self.pool)
      .await
      .map_err(|err| format!("[FileRepistory::list] Error: {}", err))?;

    Ok(files)
  }

  pub async fn create(&self, args: CreateArgs) -> Result<File, String> {
    let file = sqlx::query_as::<_, File>(
      "INSERT INTO files (directory, filename, storage_dir) VALUES (COALESCE($1, ''), $2, $3) RETURNING *",
    )
    .bind(&args.directory)
    .bind(&args.filename)
		.bind(&args.storage_dir)
    .fetch_one(&self.pool)
    .await
    .map_err(|err| format!("[FileRepistory::create] Error: {}", err))?;

    Ok(file)
  }

  pub async fn get_file(&self, args: GetFileArgs) -> Result<File, String> {
    let file = sqlx::query_as::<_, File>(
      "SELECT * FROM files WHERE filename = $1 AND directory = COALESCE($2, '') AND storage_dir = $3",
    )
    .bind(&args.filename)
    .bind(&args.directory)
		.bind(&args.storage_dir)
    .fetch_one(&self.pool)
    .await
    .map_err(|err| format!("[FileRepistory::get_file] Error: {}", err))?;

    Ok(file)
  }

  pub async fn search(&self, args: SearchArgs) -> Result<Vec<File>, String> {
    let files = sqlx::query_as::<_, File>(
      "SELECT * FROM files WHERE filename LIKE $1 AND directory = COALESCE($2, '') AND storage_dir = $3",
    )
    .bind(format!("%{}%", args.query))
		.bind(&args.storage_dir)
    .fetch_all(&self.pool)
    .await
    .map_err(|err| format!("[FileRepistory::search] Error: {}", err))?;

    Ok(files)
  }

  pub async fn delete(&self, args: DeleteArgs) -> Result<File, String> {
    let file = sqlx::query_as::<_, File>(
      "DELETE FROM files WHERE filename = $1 AND directory = COALESCE($2, '') AND storage_dir = $3 RETURNING *",
    )
    .bind(&args.filename)
    .bind(&args.directory)
		.bind(&args.storage_dir)
    .fetch_one(&self.pool)
    .await
    .map_err(|err| format!("[FileRepistory::delete] Error: {}", err))?;

    Ok(file)
  }
}

impl File {
  pub fn print_path(&self) {
    println!("{}", self.path());
  }

  pub fn print_full_path(&self) {
    println!("{}", self.full_path());
  }

  pub fn print_porcelain(&self) {
    println!("{}", self.porcelain());
  }

  pub fn path(&self) -> String {
    if !self.directory.is_empty() {
      return format!("{}/{}", self.directory, self.filename);
    }

    self.filename.clone()
  }

  pub fn full_path(&self) -> String {
    if !self.directory.is_empty() {
      return format!("{}/{}/{}", self.storage_dir, self.directory, self.filename);
    }

    format!("{}/{}", self.storage_dir, self.filename)
  }

  pub fn porcelain(&self) -> String {
    format!("{} {} {}", self.storage_dir, self.directory, self.filename)
  }
}

/// INFO: command args or subcommand args does not matter as storage_dir is global arg
async fn get_storage_dir(args: &clap::ArgMatches) -> String {
  if let Some(storage_dir) = args.try_get_one::<String>("storage-dir").unwrap() {
    return storage_dir.to_string();
  }

  let config = config::get_config(args).await;
  config.storage_dir.clone()
}
