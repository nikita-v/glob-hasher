# glob-hasher

A library that will glob for files and return the xxhash in u64 / BigInt as a JS library. It takes advantage of napi-rs to interop with the libraries doing the heavy lifting of both globbing and hashing. `xxhash-rust`, `ignore` crates are used here.
