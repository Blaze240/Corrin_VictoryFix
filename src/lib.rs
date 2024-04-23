#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod win1a;
mod win2a;
mod win3a;

#[skyline::main(name = "corrin_victoryfix_sl2")]
pub fn main() {
    win1a::install();
    win2a::install();
    win3a::install();
}