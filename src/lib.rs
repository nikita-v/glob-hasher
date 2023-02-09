#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;

use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use xxhash_rust::xxh3;

#[napi(object)]
pub struct PartialHashGlobOptions {
  pub cwd: Option<String>,
  pub gitignore: Option<bool>,
}

struct HashGlobOptions {
  pub cwd: String,
  pub gitignore: bool,
}

#[napi]
pub fn hash_glob(
  globs: Vec<String>,
  maybe_options: Option<PartialHashGlobOptions>,
) -> Option<Vec<u64>> {
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

  let HashGlobOptions { cwd, gitignore } = options;

  let mut override_builder = OverrideBuilder::new(cwd.clone());

  for glob in globs {
    override_builder.add(&glob).unwrap();
  }

  if let Ok(overrides) = override_builder.build() {
    let walker = WalkBuilder::new(cwd)
      .overrides(overrides)
      .git_ignore(gitignore)
      .build_parallel(|entry| {
        
      });

    let mut hashes: Vec<u64> = Vec::new();

    for entry in walker {
      if let Ok(entry) = entry {
        if entry.path().is_file() {
          let contents = fs::read_to_string(entry.path()).unwrap();
          hashes.push(xxh3::xxh3_64(&contents.as_bytes()));
        }
      }
    }

    return Some(hashes);
  }

  return None;
}
