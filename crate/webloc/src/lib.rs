//! This library provides the [`Webloc`] type which provides utilities for working with the `.webloc` file format macOS uses to store URLs.

#![deny(missing_docs, rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use {
    serde::{
        Deserialize,
        Serialize,
    },
    url::Url,
};

/// The contents of a `.webloc` file.
///
/// This can be read or written using the [`plist`] crate.
#[derive(Deserialize, Serialize)]
pub struct Webloc {
    /// The URL stored in the `.webloc` file.
    #[serde(rename = "URL")]
    pub url: Url,
}
