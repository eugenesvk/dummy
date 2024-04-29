use crate::helper::*; // pub use reexports for use in other files
pub use std          	::println      	as p 	; // requires text editor's syntax theme override to retain syntax highlighting
pub use std          	::eprintln     	as pe	;
pub use crate::helper	::print_type_of	as pt	;
pub use std::any     	::type_name    	     	; // for type_of

#[macro_export] macro_rules! pb { // println! during build
  ($($tokens:tt)*) => {println!("cargo:warning={}", format!($($tokens)*))}}
