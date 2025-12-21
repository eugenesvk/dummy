use std::{result, error::Error};

pub mod bpaf_ext;

pub type Result<T> = result::Result<T, Box<dyn Error>>;
pub fn ret42() -> i32 { 42 }

static _dbg:i8 = 1;
/// Quick&dirty way to disable blocks of debug-level code: `if _d(1) {}` to do something only if global _dbg ≥ 1
pub fn _d(lvl:i8) -> bool {if _dbg>=lvl{true}else{false}}
