// use nix::sched::{CloneFlags, clone};
mod image;
mod run;
mod utils;

use run::init_container;

fn main() -> () {
    init_container("ubuntu:latest");
}
