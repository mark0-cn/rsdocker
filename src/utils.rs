static GOCKER_HOME_PATH: &str = "/var/lib/gocker";
static GOCKER_TEMP_PATH: &str = "/var/lib/gocker/tmp";
static GOCKER_IMAGES_PATH: &str = "/var/lib/gocker/images";
static GOCKER_CONTAINERS_PATH: &str = "/var/run/gocker/containers";
static GOCKER_NET_NS_PATH: &str = "/var/run/gocker/net-ns";


pub fn get_rsdocker_images_path() -> String {
    GOCKER_IMAGES_PATH.to_string()
}