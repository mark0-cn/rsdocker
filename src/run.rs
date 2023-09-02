use std::process;
use nix::mount::mount;
use rand::Rng;
use crate::image::*;
use crate::utils::*;

fn get_container_fs_home(container_id: &String) -> String {
    return get_rsdocker_containers_path() + container_id + "/fs";
}

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
    let mut src_layers: Vec<String> = Vec::new();
    let path_manifest = get_manifest_path_for_image(image_sha_hex);
    let mut mani = Mainfest::new_vec();

    parse_manifest(path_manifest,&mut mani);
    if mani.len() == 0 || mani[0].layers.len() == 0 {
        log::error!("Could not find any layers.");
        process::exit(-1);
    }else if mani.len() > 1{
        log::error!("I don't know how to handle more than one manifest.");
        process::exit(-1);
    }

    let image_base_path = get_base_path_for_image(&image_sha_hex);
    for layer in &mani[0].layers {
        src_layers.push(image_base_path.clone() + &layer.chars().take(12).collect::<String>() + "/fs");
    }

    let const_fs_home = get_container_fs_home(&container_id);
    let mnt_options = format!(
        "lowerdir={}:upperdir={}/upperdir,workdir={}/workdir",
        src_layers.join(":"),
        const_fs_home,
        const_fs_home
    );
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