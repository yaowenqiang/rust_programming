#![allow(dead_code)]
#![allow(unused_imports)]
mod demo_locals;
mod demo_static_global;
use demo_static_global::GLOBAL_MESSAGE;
fn main() {
    demo_locals::do_it();
}
