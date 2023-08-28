use std::fs;

static GOCKER_HOME_PATH: &str = "/var/lib/gocker";
static GOCKER_TEMP_PATH: &str = "/var/lib/gocker/tmp";
static GOCKER_IMAGES_PATH: &str = "/var/lib/gocker/images";
static GOCKER_CONTAINERS_PATH: &str = "/var/run/gocker/containers";
static GOCKER_NET_NS_PATH: &str = "/var/run/gocker/net-ns";

pub fn init_rsdocker_dirs() -> Result<(), std::io::Error> {
    let dirs = [GOCKER_HOME_PATH, GOCKER_TEMP_PATH, GOCKER_IMAGES_PATH, GOCKER_CONTAINERS_PATH, GOCKER_NET_NS_PATH];

    for dir in dirs.iter() {
        if !fs::metadata(&dir).is_ok() {
            fs::create_dir_all(&dir)?;
        }
    }
    Ok(())
}

pub fn get_rsdocker_images_path() -> String {
    GOCKER_IMAGES_PATH.to_string()
}

pub fn get_rsdocker_tmp_path() -> String {
    GOCKER_TEMP_PATH.to_string()
}