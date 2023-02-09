#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;

use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use xxhash_rust::xxh3;

// struct HashGlobOptions {
//   cwd: String
// }

#[napi]
pub fn hash_glob(globs: Vec<String>) -> Option<Vec<u64>> {
  let mut override_builder = OverrideBuilder::new("./");

  for glob in globs {
    override_builder.add(&glob).unwrap();
  }

  if let Ok(overrides) = override_builder.build() {
    let walker = WalkBuilder::new(".")
      .overrides(overrides)
      .git_ignore(true)
      .build();

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
