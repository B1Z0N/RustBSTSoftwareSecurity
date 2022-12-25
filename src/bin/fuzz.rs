#[macro_use] extern crate afl;

use tree::runner::command_loop;

fn main() {
    fuzz!(|data: &[u8]| {
      command_loop(&mut {data});
    });
}
