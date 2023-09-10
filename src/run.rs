use std::ffi::CString;
use nix::mount::{mount, MsFlags};
use rand::Rng;
use crane::image::*;
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
    let path_manifest = get_rsdocker_tmp_path() + "/manifest.json";
    let mut mani = Manifest::new();

    parse_manifest(path_manifest,&mut mani);

    let image_base_path = get_base_path_for_image(&image_sha_hex);
    // for layer in &mani[0].layers {
    let src_layers = image_base_path.clone() + mani.etag.unwrap().as_str();
    // }

    let const_fs_home = get_container_fs_home(&container_id);
    let mnt_options = format!(
        "lowerdir={}:upperdir={}/upperdir,workdir={}/workdir",
        src_layers,
        const_fs_home,
        const_fs_home
    );

    let fstype = Some(CString::new("overlay").expect("CString::new failed"));
    let flags = MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC;
    let data = Some(CString::new(mnt_options).expect("CString::new failed"));
    let source = None;
    let target = CString::new(const_fs_home + "/mnt").expect("CString::new failed");
    if let Err(err) = mount(source.as_ref(), &target, fstype.as_ref(), flags, data.as_ref()) {
        eprintln!("Mount failed: {}", err);
        return;
    }
}

pub fn init_container(src: &str) {
    let container_id = create_container_id();
    log::info!("container_id = {}", container_id);

    let image_sha_hex = down_load_image_if_required(src);
    log::info!("Image to overlay mount: {}", image_sha_hex);

    create_container_directories(&container_id);
    mount_overlay_file_system(&container_id, &image_sha_hex);

}