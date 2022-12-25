use std::io;

use tree::runner::command_loop;


fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    command_loop(&mut handle);
}
