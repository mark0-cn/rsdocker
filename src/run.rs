use rand::Rng;
use crate::image::down_load_image_if_required;
use crate::utils::*;

fn create_container_id() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..16)
        .map(|_| rng.gen::<u8>())
        .collect();

    let container_id = random_bytes.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    container_id
}

fn create_container_directories(container_id: &String) {
    let cont_home = get_rsdocker_containers_path() + container_id;
    let cont_dirs = vec![ cont_home.clone() + "/fs", cont_home.clone() + "/fs/mnt/", cont_home.clone() + "/fs/upperdir/", cont_home.clone() + "/fs/workdir/"];

    if let Err(e) = create_dirs_if_dont_exist(&cont_dirs) {
        log::error!("{}", e);
    }
}

fn mount_overlay_file_system(container_id: &String, image_sha_hex: &String) {
    // TODO:
    todo!("mount_overlay_file_system");
}

pub fn init_container(src: &str) {
    let container_id = create_container_id();
    log::info!("container_id = {}", container_id);

    let image_sha_hex = down_load_image_if_required(src);
    log::info!("Image to overlay mount: {}", image_sha_hex);

    create_container_directories(&container_id);
    mount_overlay_file_system(&container_id, &image_sha_hex);

}