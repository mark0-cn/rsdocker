use std::collections::HashMap;
use std::fs::{self, File};
use serde_json;
use crane::image::*;
use crate::utils::*;

type ImageEntries = HashMap<String, String>;
type ImagesDB = HashMap<String, ImageEntries>;

pub fn get_base_path_for_image(image_sha_hex: &String) -> String {
    return get_rsdocker_images_path() + image_sha_hex;
}

pub fn get_manifest_path_for_image(image_sha_hex: &String) -> String {
    return get_base_path_for_image(image_sha_hex) + "/manifest.json";
}

fn get_image_name_and_tag(src: &str) -> (&str, &str) {
    let mut image_name = "";
    let mut image_tag = "";
    let mut image_name_and_tag = src.split(":");
    image_name = image_name_and_tag.next().unwrap();
    image_tag = image_name_and_tag.next().unwrap_or("latest");
    (image_name, image_tag)
}

fn parse_images_metdata(idb: &mut ImagesDB) -> () {
    let images_db_path = get_rsdocker_images_path();
    let images_db_file_path = format!("{}/images.json", images_db_path);

    if !fs::metadata(&images_db_file_path).is_ok() {
        File::create(&images_db_file_path).expect("Unable to create file");
        fs::write(&images_db_file_path, "{}").expect("Unable to write file");
    }
    let mut content = fs::read_to_string(&images_db_file_path).expect("Unable to read file");
    let tmp: ImagesDB = serde_json::from_str(&mut content).expect("Unable to parse json");
    *idb = tmp;
}

fn image_exist_by_tag(image_name: &str, tag_name: &str) -> (bool, String) {
    let idb: &mut ImagesDB = &mut HashMap::new();
    parse_images_metdata(idb);
    for (k, v) in idb {
        if k == image_name {
            for (k, v) in v {
                if k == tag_name {
                    return (true, v.to_string());
                }
            }
        }
    }
    return (false, "".to_string());
}

fn image_exists_by_hash(image_sha_hex: &String) -> (String, String) {
    let idb: &mut ImagesDB = &mut HashMap::new();
    parse_images_metdata(idb);
    for (img_name, avl_images) in idb {
        for (img_tag, img_hash) in avl_images {
            if img_hash == image_sha_hex {
                return (img_name.to_string(), img_tag.to_string());
            }
        }
    }
    return ("".to_string(), "".to_string());
}

fn marshal_image_metadata(idb: &ImagesDB) -> () {
    let file_bytes = serde_json::to_string(idb).expect("Unable to marshal json");
    let images_db_path = get_rsdocker_images_path() + "images.json";

    fs::write(&images_db_path, file_bytes).expect("Unable to write file");
}

fn store_image_metadata(image: &str, tag: &str, image_sha_hex: &String) -> () {
    let mut idb = &mut HashMap::new();
    let ientry = &mut HashMap::new();

    parse_images_metdata(&mut idb);
    if idb.contains_key(image) {
        *ientry = idb.get_mut(image).unwrap().clone();
    }
    ientry.insert(tag.to_string(), image_sha_hex.to_string());
    idb.insert(image.to_string(), ientry.to_owned());

    marshal_image_metadata(idb);
}

fn process_layer_tarballs(manifest: &crane::image::Manifest, image_sha_hex: &String, full_image_hex: &String) {
    let tmp_path_dir = get_rsdocker_tmp_path() + image_sha_hex;
    let path_manifest = get_rsdocker_tmp_path() + "/manifest.json";
    let path_config = tmp_path_dir.clone() + "/" + full_image_hex + ".json";

    let mut mani = Manifest::new();
    parse_manifest(path_manifest, &mut mani);

    // let images_dir = get_rsdocker_images_path() + image_sha_hex;
    // for layer in manifest.layers.iter() {
    //     // let image_layer_dir = images_dir + "/" + layer[:12] +"/fs";
    //     let layer_hash = layer.digest.trim_start_matches("sha256:").chars().take(64).collect::<String>();
    //     let image_layer_dir = images_dir.clone() + "/" + layer_hash.chars().take(16).collect::<String>().as_str() + "fs";

    //     log::info!("Uncompressing layer to: {}", image_layer_dir);
    //     if !fs::metadata(&image_layer_dir).is_ok() {
    //         fs::create_dir(&image_layer_dir).unwrap();
    //     }

    // }
    // TODO:
}

fn delete_temp_image_files(image_sha_hex: &String) {
    let tmp_path = get_rsdocker_tmp_path() + image_sha_hex;
    do_or_die_with_msg(fs::remove_dir_all(tmp_path).err(), "Unable to remove temporary image files");
}

fn store_manifest_metadata(manifest: &Manifest) {
    let manifest_path = get_rsdocker_tmp_path() + "manifest.json";
    let metadata = serde_json::to_string(manifest).expect("Unable to manifest json");
    fs::write(&manifest_path, metadata).expect("Unable to write file");
}

pub fn down_load_image_if_required(src: &str) -> String {
    let (img_name, tag_name) = get_image_name_and_tag(src);
    log::info!("image_name = {}, image_tag = {}", img_name, tag_name);

    let (image_is_exist, mut image_sha_hex) = image_exist_by_tag(img_name, tag_name);
    log::info!("image_is_exist = {}, image_sha_hex = {}", image_is_exist, image_sha_hex);

    if !image_is_exist {
        log::info!("Downloading metadata for {}:{}, please wait...", img_name, tag_name);
        let image: Image = Image {name: img_name.to_string(), reference: tag_name.to_string(), ..Default::default() };
        let  auth = authenticate(&image).expect("Failed to authenticate.");
        let manifest = image.get_manifest(Some(&auth.access_token)).expect("Failed to retrieve manifest.");
        let etag = manifest.etag.as_ref();
        image_sha_hex = etag.unwrap().trim_start_matches("\"sha256:").chars().take(64).collect();
        log::info!("image_sha_hex: {:#?}", image_sha_hex);
        log::info!("Checking if image exists under another name...");
        let (alt_img_name, alt_img_tag) = image_exists_by_hash(&image_sha_hex);

        if alt_img_name != "" && alt_img_tag != "" {
            log::info!("The image you requested {}:{} is the same as {}:{}", img_name, tag_name, alt_img_name, alt_img_tag);
            store_image_metadata(img_name, tag_name, &image_sha_hex);
            ()
        }
        else{
            log::info!("Image doesn't exist. Downloading...");
            let down_path = get_rsdocker_tmp_path() + &image_sha_hex;
            image.download_layers(&down_path, Some(&auth.access_token), &manifest).unwrap();
            log::info!("Successfully downloaded {}", down_path);
            // TODO:
            store_manifest_metadata(&manifest);
            // process_layer_tarballs(&manifest, &image_sha_hex, &image_sha_hex);
            store_image_metadata(img_name, tag_name, &image_sha_hex);
            // delete_temp_image_files(&image_sha_hex);
            ()
        }
    }else{
        log::info!("Image already exists. Not downloading.");
        ()
    }
    image_sha_hex
}