//! `rush` — a Unix-like shell.
//!
//! # Crate layout
//!
//! | Module | Responsibility |
//! |--------|---------------|
//! | [`shell`] | REPL loop and I/O state |
//! | [`command`] | Parsing and executing a single command |
//! | [`builtin`] | Built-in commands that run inside the shell process |
//! | [`path`] | Resolving a command name to an executable via `PATH` |
//! | [`error`] | All error types produced by this crate |

pub mod builtin;
pub mod command;
pub mod error;
pub mod path;
pub mod shell;