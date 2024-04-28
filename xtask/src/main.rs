#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
//! This binary defines various auxiliary build commands that `cargo` can't handle, e.g.:
//! - `cargo test -p xtask` for special testing
//! - `cargo xtask docgen` for for code generation
//! - `cargo xtask query-check` for query verification
//!
//! It's integrated into the `cargo` command line by using an alias in `.cargo/config`<br>
//! See [cargo-xtask](https://github.com/matklad/cargo-xtask)

use anyhow::{Result,bail};
use std::{env, path::PathBuf};
// use xshell::{cmd, Shell};

pub mod tasks {
  use anyhow::{Result,bail};
  use indoc::formatdoc;

  pub fn docgen    () -> Result<()> {Ok(())}
  pub fn querycheck() -> Result<()> {Ok(())}

  pub fn print_help() {
    let help_out = formatdoc!("
      Use: ‘cargo x <task>’, where <task> is one of ↓
        docgen     \tGenerate files to be included somewhere
        query-check\tCheck the validity of some queries"
    );
    println!("{}", help_out);
  }
}

use dummy_lib::*;
fn main() -> Result<()> {
  let task = env::args().nth(1);
  println!("task_arg1 = {:?} lib = {:?}", task, dummy_lib::lib());
  match task {
    None           	=> tasks::print_help(),
    Some(t)        	=> match t.as_str() {
      "docgen"     	=> tasks::docgen()?,
      "query-check"	=> tasks::querycheck()?,
      invalid      	=> return bail!("Invalid task name: {}", invalid),
    },
  };
  Ok(())
}
