use std::collections::HashMap;
use std::fs::{self, File};
use serde_json;
use crate::utils::get_rsdocker_images_path;

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

    if !fs::metadata(&images_db_path).is_ok() {
        fs::create_dir_all(&images_db_path).expect("Unable to create dir");
    }
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

fn store_image_metadata(image: &str, tag: &str, image_sha_hex: &String) -> () {
    let mut idb = &mut HashMap::new();
    let mut ientry = &mut HashMap::new();

    parse_images_metdata(&mut idb);
    if idb.contains_key(image) {
        ientry = idb.get_mut(image).unwrap();
    }
    // ientry.insert(tag.to_string(), *image_sha_hex);
    // idb.insert(image.to_string(), ientry.to_owned());
}

pub fn down_load_image_if_required(src: &str) -> String {
    let (img_name, tag_name) = get_image_name_and_tag(src);
    println!("image_name = {}, image_tag = {}", img_name, tag_name);

    let (down_load_required, image_sha_hex) = image_exist_by_tag(img_name, tag_name);
    println!("down_load_required = {}, image_sha_hex = {}", down_load_required, image_sha_hex);

    if !down_load_required {
        println!("Downloading metadata for {}:{}, please wait...", img_name, tag_name);
        // img, err := crane.Pull(strings.Join([]string{imgName, tagName}, ":"))
		// if err != nil {
		// 	log.Fatal(err)
		// }

		// manifest, _ := img.Manifest()
		// imageShaHex = manifest.Config.Digest.Hex[:12]
		// log.Printf("imageHash: %v\n", imageShaHex)
        println!("image_hash = {}", image_sha_hex);
        println!("Checking if image exists under another name...");
        let (alt_img_name, alt_img_tag) = image_exists_by_hash(&image_sha_hex);
        if alt_img_name != "" && alt_img_tag != "" {
            println!("The image you requested {}:{} is the same as {}:{}", img_name, tag_name, alt_img_name, alt_img_tag);
            store_image_metadata(img_name, tag_name, &image_sha_hex);
            
        }
    }else{
        println!("Image already exists. Not downloading.");
        return image_sha_hex
    }
    todo!("todo!!!");
    image_sha_hex
}