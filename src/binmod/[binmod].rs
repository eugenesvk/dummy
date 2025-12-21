extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;
// use crate::*;
pub use dummy_lib::libmod::*;

pub mod cli_opt;
use cli_opt::*;

pub fn print42() -> Result<()> {p!("{}",42)?; Ok(())}

use std::path::PathBuf;
use docpos::*;

#[docpos] pub struct StructyPos { /// "inner" scruct docs
  pub field1       :        String  ,/// pos-doc for `field1` (in regular Rust this would be a doc for `field2_longer`)
  pub field2_longer: Option<String> ,/// pos-doc for `field2_longer`
                                     /// pos-doc for `field2_longer` line 2
                                     ///! pre-doc for `paths` at `field2_longer` (after `///!`)
  pub paths        : Vec   <PathBuf>, // no doc comments allowed here, use `///!` in the previous field
}

pub fn main_cli() -> Result<()> {
  let opt = options().run();
  p!("got the followin arguments: fmt={}\nlog={}\npaths={:?}",opt.fmt,opt.log,opt.paths)?;
  Ok(())
}
