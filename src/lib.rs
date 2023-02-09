#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::collections::HashMap;
use std::fs;
use std::thread;

use ignore::overrides::OverrideBuilder;
use ignore::{DirEntry, WalkBuilder};
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

    let (tx, rx) = crossbeam_channel::bounded::<(DirEntry, u64)>(100);

    // let collect_thread = thread::spawn(move || {
    //   for (dir_entry, hash) in rx {
    //     hashes.insert(dir_entry.path().to_string_lossy().to_string(), hash);
    //   }
    // });

    WalkBuilder::new(cwd)
      .overrides(overrides)
      .git_ignore(gitignore)
      .threads(6)
      .build_parallel()
      .run(|| {
        let tx = tx.clone();
        Box::new(move |dir_entry_result| {
          use ignore::WalkState::*;
          if let Ok(dir_entry) = dir_entry_result {
            if dir_entry.path().is_file() {
              let contents = fs::read_to_string(dir_entry.path()).unwrap();
              tx.send((dir_entry, xxh3::xxh3_64(&contents.as_bytes())));
            }
          }
          Continue
        })
      });

    // for entry in walker {
    //   if let Ok(entry) = entry {
    //     if entry.path().is_file() {
    //       let contents = fs::read_to_string(entry.path()).unwrap();
    //       hashes.push(xxh3::xxh3_64(&contents.as_bytes()));
    //     }
    //   }
    // }


    for (dir_entry, hash) in rx.try_iter().collect() {
      hashes.insert(dir_entry.path().to_string_lossy().to_string(), hash);
    }

    drop(tx);
    collect_thread.join().unwrap();

    // let hashes = hashes;

    return Some(hashes);
  }

  return None;
}
