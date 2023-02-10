#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod config;
mod hasher;

pub mod glob_hasher {
  use crate::config::{
    get_hash_glob_config, PartialHashGlobOptions, PartialHashGlobParallelOptions, get_hash_glob_parallel_config,
  };
  use crate::hasher;
  use std::collections::HashMap;

  #[napi]
  pub fn hash_glob(
    globs: Vec<String>,
    maybe_options: Option<PartialHashGlobOptions>,
  ) -> Option<HashMap<String, u64>> {
    let options = get_hash_glob_config(maybe_options);
    hasher::serial(globs, options)
  }

  #[napi]
  pub fn hash_glob_parallel(
    globs: Vec<String>,
    maybe_options: Option<PartialHashGlobParallelOptions>,
  ) -> Option<HashMap<String, u64>> {
    let options = get_hash_glob_parallel_config(maybe_options);
    hasher::parallel(globs, options)
  }
}
