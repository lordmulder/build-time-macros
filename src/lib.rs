// SPDX-License-Identifier: 0BSD
// build-time-macros
// Copyright (C) 2026 by LoRd_MuldeR <mulder2@gmx.de>

//! Provides simple procedural macros to access the build date and time at compile time.
//!
//! ### Usage
//!
//! Example usage:
//! ```rust
//! use build_time_macros::{build_date, build_full, build_time, build_unix};
//!
//! fn main() {
//!     println!("Build date : {}", build_date!());
//!     println!("Build time : {}", build_time!());
//!     println!("Build full : {}", build_full!());
//!     println!("Build unix : {}", build_unix!());
//! }
//! ```
//!
//! Example output:
//! ```text
//! Build date : 2026-11-20
//! Build time : 14:23:34
//! Build full : 2026-11-20T14:23:34Z
//! Build unix : 1795184614
//! ```
//!
//! ### Features
//!
//! Optional feature `local` enables support for local time-zone output &#x1F4CD;&#x1F552;

use proc_macro::{Literal, TokenStream, TokenTree};
use std::sync::LazyLock;
use time::{
    format_description::{well_known, BorrowedFormatItem},
    macros::format_description,
    OffsetDateTime,
};

/// Time format `HH:MM:SS`
const FORMAT_DESCR_TIME: &[BorrowedFormatItem<'_>] = format_description!("[hour]:[minute]:[second]");

/// Date format `YYYY-MM-DD`
const FORMAT_DESCR_DATE: &[BorrowedFormatItem<'_>] = format_description!("[year]-[month]-[day]");

/// Timestamp, captured once
static TIMESTAMP: LazyLock<OffsetDateTime> = LazyLock::new(|| OffsetDateTime::now_utc().truncate_to_second());

// ----------------------------------------------------------------
// UTC time
// ----------------------------------------------------------------

/// The build time (UTC) as a string literal, formatted as `HH:MM:SS`.
#[proc_macro]
pub fn build_time(_: TokenStream) -> TokenStream {
    let time_str = TIMESTAMP.format(FORMAT_DESCR_TIME).unwrap();
    TokenTree::Literal(Literal::string(&time_str)).into()
}

/// The build date (UTC) as a string literal, formatted as `YYYY-MM-DD`.
#[proc_macro]
pub fn build_date(_: TokenStream) -> TokenStream {
    let date_str = TIMESTAMP.format(FORMAT_DESCR_DATE).unwrap();
    TokenTree::Literal(Literal::string(&date_str)).into()
}

/// The build date and time (UTC) as a string literal, formatted as [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html).
#[proc_macro]
pub fn build_full(_: TokenStream) -> TokenStream {
    let date_str = TIMESTAMP.format(&well_known::Rfc3339).unwrap();
    TokenTree::Literal(Literal::string(&date_str)).into()
}

/// The build date and time as a [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time), represented as `i64` value.
#[proc_macro]
pub fn build_unix(_: TokenStream) -> TokenStream {
    let unix_ts = TIMESTAMP.unix_timestamp();
    TokenTree::Literal(Literal::i64_suffixed(unix_ts)).into()
}

// ----------------------------------------------------------------
// Local time support
// ----------------------------------------------------------------

/// Convert UTC timestamp to local time
#[cfg(feature = "local")]
static LOCALTIME: LazyLock<OffsetDateTime> = LazyLock::new(|| {
    let offset = time::UtcOffset::local_offset_at(*TIMESTAMP).unwrap_or(time::UtcOffset::UTC);
    TIMESTAMP.to_offset(offset)
});

/// The build time (local timezone) as a string literal, formatted as `HH:MM:SS`.
#[cfg(feature = "local")]
#[proc_macro]
pub fn local_time(_: TokenStream) -> TokenStream {
    let time_str = LOCALTIME.format(FORMAT_DESCR_TIME).unwrap();
    TokenTree::Literal(Literal::string(&time_str)).into()
}

/// The build date (local timezone) as a string literal, formatted as `YYYY-MM-DD`.
#[cfg(feature = "local")]
#[proc_macro]
pub fn local_date(_: TokenStream) -> TokenStream {
    let date_str = LOCALTIME.format(FORMAT_DESCR_DATE).unwrap();
    TokenTree::Literal(Literal::string(&date_str)).into()
}

/// The build date and time (local timezone) as a string literal, formatted as [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html).
#[cfg(feature = "local")]
#[proc_macro]
pub fn local_full(_: TokenStream) -> TokenStream {
    let date_str = LOCALTIME.format(&well_known::Rfc3339).unwrap();
    TokenTree::Literal(Literal::string(&date_str)).into()
}
