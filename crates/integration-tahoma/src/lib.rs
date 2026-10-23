//! Bounded, explicitly authenticated TaHoma discovery and HTTP transport.
pub mod client;
pub mod discovery;
pub mod dto;
pub use client::{TahomaClient, TahomaCredentials};
pub use discovery::discover;
