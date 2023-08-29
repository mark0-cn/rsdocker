use std::collections::HashMap;
use std::fs::{self, File};
use serde_json;
use crate::utils::*;
use crate::tarfile::*;

type ImageEntries = HashMap<String, String>;
type ImagesDB = HashMap<String, ImageEntries>;

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
    let images_db_path = get_rsdocker_images_path() + "/images.json";

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

fn down_load_image(img: &str, image_sha_hex: &String, src: &str) -> () {
    let mut path = get_rsdocker_tmp_path() + "/" + &image_sha_hex;
    path += "/package.tar";
    // TODO:
    // if err := crane.SaveLegacy(img, src, path); err != nil {
	// 	log.Fatalf("saving tarball %s: %v", path, err)
	// }
    log::info!("Successfully downloaded {}", src);
}

fn untar_file(image_sha_hex: &String) {
    let path_dir = get_rsdocker_tmp_path() + "/" + image_sha_hex;
    let path_tar = path_dir.as_str().to_owned() + "package.tar";

    log::info!("untar_file path_tar = {}, dest_path = {}, ", path_tar, path_dir);
    
    match untar(&path_tar, &path_dir) {
        Ok(_) => {
            log::info!("Successfully untar {}", path_tar);
        },
        Err(e) => {
            log::error!("Error untaring file: {}", e);
        }
    }
}

fn process_layer_tarballs(image_sha_hex: &String, full_image_hex: &String) {
    todo!("todo!!!");
}

pub fn down_load_image_if_required(src: &str) -> String {
    let (img_name, tag_name) = get_image_name_and_tag(src);
    log::info!("image_name = {}, image_tag = {}", img_name, tag_name);

    let (image_is_exist, image_sha_hex) = image_exist_by_tag(img_name, tag_name);
    log::info!("image_is_exist = {}, image_sha_hex = {}", image_is_exist, image_sha_hex);

    if !image_is_exist {
        log::info!("Downloading metadata for {}:{}, please wait...", img_name, tag_name);
        // img, err := crane.Pull(strings.Join([]string{imgName, tagName}, ":"))
		// if err != nil {
		// 	log.Fatal(err)
		// }

		// manifest, _ := img.Manifest()
		// imageShaHex = manifest.Config.Digest.Hex[:12]
		// log.Printf("imageHash: %v\n", imageShaHex)
        log::info!("image_hash = {}", image_sha_hex);
        log::info!("Checking if image exists under another name...");
        let (alt_img_name, alt_img_tag) = image_exists_by_hash(&image_sha_hex);
        if alt_img_name != "" && alt_img_tag != "" {
            log::info!("The image you requested {}:{} is the same as {}:{}", img_name, tag_name, alt_img_name, alt_img_tag);
            store_image_metadata(img_name, tag_name, &image_sha_hex);
            return image_sha_hex;
        }
        else{
            log::info!("Image doesn't exist. Downloading...");
            down_load_image("", &image_sha_hex, src);
            untar_file(&image_sha_hex);
            // process_layer_tarballs(&image_sha_hex, manifest.Config.Digest.Hex);
            // store_image_metadata(img_name, tag_name, &image_sha_hex);
            // delete_temp_image_files(image_sha_hex);
            return image_sha_hex
        }
    }else{
        log::info!("Image already exists. Not downloading.");
        return image_sha_hex
    }
    todo!("todo!!!");
    image_sha_hex
}