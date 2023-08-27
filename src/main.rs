mod image;
mod run;
mod utils;

use std::env;
use nix::unistd::geteuid;
use run::init_container;
use utils::init_rsdocker_dirs;

fn usage() -> () {
    println!("Welcome to Rsdocker!");
    println!("Supported commands:");
    println!("rsdocker run [--mem] [--swap] [--pids] [--cpus] <image> <command>");
    println!("rsdocker exec <container-id> <command>");
    println!("rsdocker images");
    println!("rsdocker rmi <image-id>");
    println!("rsdocker ps");
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let options = ["run", "child-mode", "setup-netns", "setup-veth", "ps", "exec", "images", "rmi"];

    // 检查参数数量
    if args.len() < 2 || !options.contains(&args[1].as_str()) {
        usage();
        return;
    }

    if geteuid().is_root() == false {
        println!("You need root privileges to run this program.");
        return;
    }

    init_rsdocker_dirs().expect("Unable to create requisite directories");
    match args[1].as_str() {
        "run" => {
            init_container("alpine");
        },
        "exec" => {
            todo!("exec");
        },
        "images" => {
            todo!("images");
        },
        "rmi" => {
            todo!("rmi");
        },
        "ps" => {
            todo!("ps");
        },
        _ => {
            usage();
        }
    }
        
}
