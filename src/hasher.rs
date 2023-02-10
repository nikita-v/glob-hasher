use dashmap::DashMap;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::{collections::HashMap, sync::Arc};
use std::fs;
use xxhash_rust::xxh3;

use crate::config::{HashGlobOptions, HashGlobParallelOptions};

pub fn serial(globs: Vec<String>, options: HashGlobOptions) -> Option<HashMap<String, u64>> {
  let HashGlobOptions { cwd, gitignore } = options;

  let mut override_builder = OverrideBuilder::new(cwd.clone());

  for glob in globs {
    override_builder.add(&glob).unwrap();
  }

  if let Ok(overrides) = override_builder.build() {
    let mut hashes: HashMap<String, u64> = HashMap::new();

    let walker = WalkBuilder::new(&cwd)
      .overrides(overrides)
      .git_ignore(gitignore)
      .build();

    for dir_entry_result in walker {   
      if let Ok(dir_entry) = dir_entry_result {
        if dir_entry.path().is_file() {
          let contents = fs::read_to_string(dir_entry.path()).unwrap();
          hashes.insert(
            dir_entry.path().strip_prefix(&cwd).unwrap().to_string_lossy().to_string(),
            xxh3::xxh3_64(&contents.as_bytes()),
          );
        }
      }
    }

    return Some(hashes);
  }

  return None;
}

pub fn parallel(
  globs: Vec<String>,
  options: HashGlobParallelOptions,
) -> Option<HashMap<String, u64>> {
  let HashGlobParallelOptions {
    cwd,
    gitignore,
    concurrency,
  } = options;

  let mut override_builder = OverrideBuilder::new(cwd.clone());

  for glob in globs {
    override_builder.add(&glob).unwrap();
  }

  if let Ok(overrides) = override_builder.build() {
    let hashes = Arc::new(DashMap::<String, u64>::new());

    WalkBuilder::new(&cwd)
      .overrides(overrides)
      .git_ignore(gitignore)
      .threads(concurrency)
      .build_parallel()
      .run(|| {
        let map = hashes.clone();
        let base_cwd = cwd.clone();
        Box::new(move |dir_entry_result| {
          use ignore::WalkState::*;

          if let Ok(dir_entry) = dir_entry_result {
            if dir_entry.path().is_file() {
              let contents = fs::read_to_string(dir_entry.path()).unwrap();
              map.insert(
                dir_entry.path().strip_prefix(&base_cwd).unwrap().to_string_lossy().to_string(),
                xxh3::xxh3_64(&contents.as_bytes()),
              );
            }
          }
          Continue
        })
      });

    if let Ok(map) = Arc::try_unwrap(hashes) {
      return Some(map.into_iter().collect::<HashMap<String, u64>>());
    }
  }

  return None;
}
