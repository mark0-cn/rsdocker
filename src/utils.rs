use std::fs::{self, File};
use std::io::prelude::*;
use std::error::Error;
use crate::image::*;

static RSDOCKER_HOME_PATH: &str = "/var/lib/rsdocker/";
static RSDOCKER_TEMP_PATH: &str = "/var/lib/rsdocker/tmp/";
static RSDOCKER_IMAGES_PATH: &str = "/var/lib/rsdocker/images/";
static RSDOCKER_CONTAINERS_PATH: &str = "/var/run/rsdocker/containers/";
static RSDOCKER_NET_NS_PATH: &str = "/var/run/rsdocker/net-ns/";

pub fn init_rsdocker_dirs() -> Result<(), std::io::Error> {
    let dirs = [RSDOCKER_HOME_PATH, RSDOCKER_TEMP_PATH, RSDOCKER_IMAGES_PATH, RSDOCKER_CONTAINERS_PATH, RSDOCKER_NET_NS_PATH];

    for dir in dirs.iter() {
        if !fs::metadata(&dir).is_ok() {
            fs::create_dir_all(&dir)?;
        }
    }
    Ok(())
}

pub fn do_or_die_with_msg(err: Option<impl Error> , msg: &str) {
    match err {
        Some(_err) => {
            println!("{}", msg);
        },
        None => {

        }
    }
}

pub fn create_dirs_if_dont_exist(dirs: &[String]) -> Result<(), std::io::Error> {
    for dir in dirs {
        if !fs::metadata(dir).is_ok() {
            fs::create_dir_all(&dir)?;
        }
    }
    Ok(())
}

pub fn get_rsdocker_images_path() -> String {
    RSDOCKER_IMAGES_PATH.to_string()
}

pub fn get_rsdocker_tmp_path() -> String {
    RSDOCKER_TEMP_PATH.to_string()
}

pub fn get_rsdocker_containers_path() -> String {
    RSDOCKER_CONTAINERS_PATH.to_string()
}

pub fn parse_manifest(manifest_path: String, mani: &mut Vec<Mainfest>) {
    let mut file = File::open(manifest_path).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content);
    let serde_mani: Vec<Mainfest> = serde_json::from_str(content.as_str()).expect("Unable prase json");
    *mani = serde_mani;
}