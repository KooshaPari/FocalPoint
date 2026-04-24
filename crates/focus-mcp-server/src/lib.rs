//! focus-mcp-server library exports for testing and integration.

pub mod server;
pub mod tools;

pub use server::{run_stdio, run_sse};
pub use tools::FocalPointToolsImpl;
