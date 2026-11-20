# build-time-macros

[![Crates.io](https://img.shields.io/crates/v/build-time-macros)](https://crates.io/crates/build-time-macros)
[![Downloads](https://img.shields.io/crates/d/build-time-macros)](https://crates.io/crates/build-time-macros)
[![Release Date](https://img.shields.io/github/release-date/lordmulder/build-time-macros)](https://crates.io/crates/build-time-macros/versions)
[![Docs.rs](https://img.shields.io/docsrs/build-time-macros)](https://docs.rs/build-time-macros/latest/)
[![License](https://img.shields.io/crates/l/build-time-macros)](https://opensource.org/license/0BSD)

Provides simple procedural macros to access the build date and time at compile time.

### Installation

Add this crate under `[dependencies]` to your **`Cargo.toml`**:

```
[dependencies]
build-time-macros = "1.0.0"
```

### Usage

Example usage:
```rust
use build_time_macros::{build_date, build_full, build_time, build_unix};

fn main() {
    println!("Build date : {}", build_date!());
    println!("Build time : {}", build_time!());
    println!("Build full : {}", build_full!());
    println!("Build unix : {}", build_unix!());
}
```

Example output:
```text
Build date : 2026-11-20
Build time : 14:23:34
Build full : 2026-11-20T14:23:34Z
Build unix : 1795184614
```

### Features

Optional feature `local` enables support for local time-zone output &#x1F4CD;&#x1F552;

## License

This software is released under the BSD Zero Clause (“0BSD”) License.

Copyright (C) 2026 by LoRd_MuldeR &lt;mulder2@gmx.de&gt;.
