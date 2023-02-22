use dashmap::DashMap;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::fs;
use std::path::Path;
use std::{collections::HashMap, sync::Arc};
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
      .hidden(false)
      .build();

    for dir_entry_result in walker {
      if let Ok(dir_entry) = dir_entry_result {
        if dir_entry.path().is_file() {
          let contents = read_file(dir_entry.path()).expect("Failed to read file");
          hashes.insert(
            dir_entry
              .path()
              .strip_prefix(&cwd)
              .unwrap()
              .to_string_lossy()
              .to_string(),
            xxh3::xxh3_64(&contents),
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
      .hidden(false)
      .threads(concurrency)
      .build_parallel()
      .run(|| {
        let map = hashes.clone();
        let base_cwd = cwd.clone();
        Box::new(move |dir_entry_result| {
          use ignore::WalkState::*;

          if let Ok(dir_entry) = dir_entry_result {
            if dir_entry.path().is_file() {
              let contents = read_file(dir_entry.path()).expect("Failed to read file");
              map.insert(
                dir_entry
                  .path()
                  .strip_prefix(&base_cwd)
                  .unwrap()
                  .to_string_lossy()
                  .to_string(),
                xxh3::xxh3_64(&contents),
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

fn read_file(path: &Path) -> Result<Vec<u8>, anyhow::Error> {
  let read_bytes = fs::read(path);

  let mut result = Vec::new();
  let mut prev_byte: Option<u8> = None;

  if let Ok(ref bytes) = read_bytes {
    if is_binary(bytes) {
      return Ok(bytes.clone());
    }
  }

  if let Ok(ref bytes) = read_bytes {
    for byte in bytes.clone() {
      match (prev_byte, byte) {
        (Some(b'\r'), b'\n') => {}
        (None, _) => {}
        (Some(ref b), _) => {
          // println!("normal byte: {:?}", b as char);
          // Not a CRLF, add previous byte to output vector
          result.push(b.clone());
        }
      }

      prev_byte = Some(byte);
    }

    // Add last byte to output vector if it was not a CR
    if let Some(byte) = prev_byte {
      if byte != b'\r' {
        result.push(byte);
      }
    }
  } else {
    return Err(anyhow::anyhow!("Failed to read file"));
  }

  Ok(result)
}

// Same rule as git in detecting whether or not the file is binary
fn is_binary(bytes: &Vec<u8>) -> bool {
  let first_few_bytes = bytes.iter().take(8000);
  for byte in first_few_bytes {
    if byte == &b'\0' {
      return true;
    }
  }

  false
}
