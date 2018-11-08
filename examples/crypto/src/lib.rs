#![deny(warnings)]
#![deny(unused_extern_crates)]

extern crate static_atom;

include!(concat!(env!("OUT_DIR"), "/atoms.rs"));
