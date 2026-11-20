// SPDX-License-Identifier: 0BSD
// build-time-macros
// Copyright (C) 2026 by LoRd_MuldeR <mulder2@gmx.de>

use build_time_macros::{build_date, build_full, build_time, build_unix};

#[cfg(feature = "local")]
use build_time_macros::{local_date, local_full, local_time};

/// Example usage of build of [`build_time_macros`].
fn main() {
    println!("Build date : {}", build_date!());
    println!("Build time : {}", build_time!());
    println!("Build full : {}", build_full!());
    println!("Build unix : {}", build_unix!());

    #[cfg(feature = "local")]
    {
        println!("Local date : {}", local_date!());
        println!("Local time : {}", local_time!());
        println!("Local full : {}", local_full!());
    }
}
