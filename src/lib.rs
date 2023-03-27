#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod config;
mod glob;
mod hasher;

pub mod glob_hasher {
  use crate::config::{get_hash_glob_config, PartialHashGlobOptions};
  use crate::glob::glob as glob_fn;
  use crate::hasher;
  use std::collections::HashMap;
  use std::path::PathBuf;

  #[napi]
  pub fn hash_glob_xxhash(
    globs: Vec<String>,
    maybe_options: Option<PartialHashGlobOptions>,
  ) -> Option<HashMap<String, u64>> {
    let options = get_hash_glob_config(maybe_options);

    if let Some(concurrency) = options.concurrency {
      rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency)
        .build_global()
        .unwrap_or_default();
    }

    match glob_fn(globs, &options) {
      Some(file_set) => hasher::xxhash(file_set, options.cwd.as_str()),
      None => None,
    }
  }

  #[napi]
  pub fn hash_glob_git(
    globs: Vec<String>,
    maybe_options: Option<PartialHashGlobOptions>,
  ) -> Option<HashMap<String, String>> {
    let options = get_hash_glob_config(maybe_options);

    if let Some(concurrency) = options.concurrency {
      rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency)
        .build_global()
        .unwrap_or_default();
    }

    match glob_fn(globs, &options) {
      Some(file_set) => hasher::git_hash(file_set, options.cwd.as_str()),
      None => None,
    }
  }

  #[napi]
  pub fn hash(
    files: Vec<String>,
    maybe_options: Option<PartialHashGlobOptions>,
  ) -> Option<HashMap<String, String>> {
    let options = get_hash_glob_config(maybe_options);

    if let Some(concurrency) = options.concurrency {
      rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency)
        .build_global()
        .unwrap_or_default();
    }

    let file_set: Vec<PathBuf> = files.iter().map(|f| PathBuf::from(&f)).collect();

    hasher::git_hash_vec(file_set, options.cwd.as_str())
  }

  #[napi]
  pub fn glob(
    globs: Vec<String>,
    maybe_options: Option<PartialHashGlobOptions>,
  ) -> Option<Vec<String>> {
    let options = get_hash_glob_config(maybe_options);
    match glob_fn(globs, &options) {
      Some(file_set) => Some(
        file_set
          .into_iter()
          .map(|path_buf| path_buf.into_os_string().to_string_lossy().to_string())
          .collect(),
      ),
      None => None,
    }
  }
}
