//! This library provides the [`Webloc`] type which provides utilities for working with the `.webloc` file format macOS uses to store URLs.

use {
    semver::Version,
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

#[doc(hidden)]
pub fn version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).expect("failed to parse current version")
}
