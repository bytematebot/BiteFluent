// OAuth providers.
//
// Concrete providers will live here, for example:
// - github
// - discord
// - google
// - custom

pub mod github;
pub use github::GitHubProvider;
