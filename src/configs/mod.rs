//! # Info module, used to store configuration information.
//! Configs like the VTEX account, the user email, and the session token.
//!
//! # Examples
//! ```rust
//! let session = get_session(path);
//! ```
//! # Panics
//! This function will panic if the session file is not properly formatted.
//! This is because the session file will not be able to be deserialized.

/// # Project struct, used to store project information.
pub mod project;
pub use project::Project;

/// # Auth module, used to store authentication information.
pub mod vtex;
pub use vtex::Vtex;
