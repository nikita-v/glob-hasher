#[napi(object)]
pub struct PartialHashGlobOptions {
  pub cwd: Option<String>,
  pub gitignore: Option<bool>,
}

pub struct HashGlobOptions {
  pub cwd: String,
  pub gitignore: bool,
}

pub fn get_hash_glob_config(maybe_options: Option<PartialHashGlobOptions>) -> HashGlobOptions {
  let mut options = HashGlobOptions {
    cwd: ".".to_string(),
    gitignore: true,
  };

  if let Some(passed_in_options) = maybe_options {
    if let Some(cwd) = passed_in_options.cwd {
      options.cwd = cwd;
    }

    if let Some(gitignore) = passed_in_options.gitignore {
      options.gitignore = gitignore;
    }
  }

  options
}

#[napi(object)]
pub struct PartialHashGlobParallelOptions {
  pub cwd: Option<String>,
  pub gitignore: Option<bool>,
  pub concurrency: Option<u32>,
}

pub struct HashGlobParallelOptions {
  pub cwd: String,
  pub gitignore: bool,
  pub concurrency: usize,
}

pub fn get_hash_glob_parallel_config(maybe_options: Option<PartialHashGlobParallelOptions>) -> HashGlobParallelOptions {
  let mut options = HashGlobParallelOptions {
    cwd: ".".to_string(),
    gitignore: true,
    concurrency: 4,
  };

  if let Some(passed_in_options) = maybe_options {
    if let Some(cwd) = passed_in_options.cwd {
      options.cwd = cwd;
    }

    if let Some(gitignore) = passed_in_options.gitignore {
      options.gitignore = gitignore;
    }
  }

  options
}
