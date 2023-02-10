#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use dashmap::DashMap;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::thread;
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
pub fn hash_glob_parallel(
  globs: Vec<String>,
  maybe_options: Option<PartialHashGlobOptions>,
) -> Option<HashMap<String, u64>> {
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
    let hashes = Arc::new(DashMap::<String, u64>::new());

    // let (tx, rx) = crossbeam_channel::bounded(100);

    // let stdout_thread = thread::spawn(move || {
    //   for (dir_entry, hash) in rx {
    //     println!("{:?}, {:?}", dir_entry, hash);
    //   }
    // });

    WalkBuilder::new(cwd)
      .overrides(overrides)
      .git_ignore(gitignore)
      .threads(100)
      .build_parallel()
      .run(|| {
        // let tx = tx.clone();
        let map = hashes.clone();
        Box::new(move |dir_entry_result| {
          use ignore::WalkState::*;

          if let Ok(dir_entry) = dir_entry_result {
            if dir_entry.path().is_file() {
              let contents = fs::read_to_string(dir_entry.path()).unwrap();
              map.insert(
                dir_entry.path().to_string_lossy().to_string(),
                xxh3::xxh3_64(&contents.as_bytes()),
              );
              // tx.send((dir_entry, xxh3::xxh3_64(&contents.as_bytes())))
              //   .unwrap();
            }
          }
          Continue
        })
      });

    // drop(tx);
    // stdout_thread.join().unwrap();

    if let Ok(map) = Arc::try_unwrap(hashes) {
      return Some(map.into_iter().collect::<HashMap<String, u64>>());
    }
  }

  return None;
}

#[napi]
pub fn hash_glob(
  globs: Vec<String>,
  maybe_options: Option<PartialHashGlobOptions>,
) -> Option<HashMap<String, u64>> {
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
    let mut hashes: HashMap<String, u64> = HashMap::new();

    let walker = WalkBuilder::new(cwd)
      .overrides(overrides)
      .git_ignore(gitignore)
      .build();

    for dir_entry_result in walker {
      if let Ok(dir_entry) = dir_entry_result {
        if dir_entry.path().is_file() {
          let contents = fs::read_to_string(dir_entry.path()).unwrap();
          hashes.insert(
            dir_entry.path().to_string_lossy().to_string(),
            xxh3::xxh3_64(&contents.as_bytes()),
          );
        }
      }
    }

    return Some(hashes);
  }

  return None;
}
