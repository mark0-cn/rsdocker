use rand::Rng;
use crate::image::down_load_image_if_required;

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

pub fn init_container(src: &str) {
    let container_id = create_container_id();
    log::info!("container_id = {}", container_id);

    let image_sha_hex = down_load_image_if_required(src);
}