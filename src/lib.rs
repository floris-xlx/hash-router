//! Hash router for routing internal ID's to services
//!
//! Still a WIP but is made to efficiently map discord id's and trade hashes to services when providing discord message signals
//!
//! ## Supported service identities
//! - Discord message ID's
//! - Discord channel ID's
//! - Discord guild ID's
//! - Trade hashes
//!
//!
//! ## Table of contents
//! - [Router](./router/index.html)
//! - [Model](./model/index.html)
//! - [Databasing](./db/index.html)
//! - [Errors](./errors/index.html)
//! - [Success](./success/index.html)
//!
//!
//!

pub mod errors;
pub mod db;
pub mod utils;
pub mod success;
pub mod model;
pub mod router;
