mod image;
mod run;
mod tarfile;
mod utils;

use std::{env, process};
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

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // 检查参数数量
    if args.len() < 2 || !options.contains(&args[1].as_str()) {
        log::error!("Args length is less than 2 or invalid option");
        usage();
        process::exit(1);
    }

    if geteuid().is_root() == false {
        println!("You need root privileges to run this program.");
        process::exit(1);
    }
    log::info!("args: {:?}", args);
    
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
