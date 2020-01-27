#![allow(dead_code)] //ignores dead code for all submodules
#![allow(unused_macros)] //ignores dead macros for all submodules
extern crate serde;
extern crate itertools;
pub mod recipes;

#[no_mangle]
pub extern "C" fn rsum(n1: u32, n2: u32) -> u32 {
    n1 + n2
}