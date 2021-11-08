#![feature(cfg_target_abi)]
#![cfg_attr(target_abi = "none", no_std)]

#[cfg(not(target_abi = "none"))]
fn main() {
    println!("Hello, world!");
}
