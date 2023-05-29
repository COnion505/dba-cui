mod common;
mod console;
use common::input_content;

// main
fn main() {
    input_content().and_then(console::run).expect("can't run.");
}
