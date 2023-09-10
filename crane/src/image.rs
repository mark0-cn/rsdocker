use std::path::{Path, PathBuf};
use std::io::Cursor;
use std::fs::{create_dir_all, File};
use serde::{Deserialize, Serialize};
use reqwest::header::{CONTENT_TYPE, ETAG};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Image {
    pub registry: String,
    pub repository: String,
    pub name: String,
    pub reference: String,
}
impl Default for Image {
    fn default() -> Image {
        Image {
            registry: String::from("registry.hub.docker.com"),
            repository: String::from("library"),
            name: String::from(""),
            reference: String::from(""),
        }
    }
}

impl Image {
    pub fn get_manifest(&self, token: Option<&String>) -> Result<Manifest, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let builder = client.get(format!(
            "https://{}/v2/{}/{}/manifests/{}",
            self.registry, self.repository, self.name, self.reference
        ));
        let builder = match token {
            Some(token) => {
                builder.header("Authorization", format!("Bearer {}", &token))
            },
            None => builder,
        };
        let response = builder.send()?;
        let headers = response.headers().clone();
        let mut manifest: Manifest;

        if let Some(content_type) = headers.get(CONTENT_TYPE) {
            match content_type.to_str() {
                Ok(content_type_str) => {
                    if content_type_str == "application/vnd.oci.image.index.v1+json" {
                        // manifest list type
                        // TODO:
                        todo!("manifest list")
                    } else if content_type_str == "application/vnd.docker.distribution.manifest.v1+prettyjws" {
                        // image manifest type
                        manifest = response.json()?;
                    } else {
                        panic!("Received content with unknown or different Content-Type.");
                    }
                }
                Err(_) => {
                    panic!("Invalid Content-Type header.");
                }
            }
        } else {
            panic!("No Content-Type header found.");
        }

        manifest.etag = Some(String::from(headers.get(ETAG).unwrap().to_str().unwrap()).trim_start_matches("\"sha256:").chars().take(64).collect::<String>());
        Ok(manifest)
    }

    pub fn download_layers(
        &self,
        path: &str,
        token: Option<&String>,
        manifest: &Manifest
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(path);
        create_dir_all(path)?;
        let layer_dir = tempfile::tempdir().expect("Failed to create temporary directory.");
        let layer_file_path = PathBuf::from(layer_dir.path()).join("layer.tar");
        let client = reqwest::blocking::Client::new();
        for layer in manifest.layers.iter() {
            let builder = client.get(&format!(
                "https://{}/v2/{}/{}/blobs/{}",
                self.registry, self.repository, self.name, layer.digest
            ));
            let builder = match token {
                Some(token) => builder.bearer_auth(token),
                None => builder,
            };
            let response = builder.send().expect("Failed to get layer.");
            let mut content = Cursor::new(response.bytes().expect("Failed to get content"));
            let mut file = File::create(&layer_file_path).expect("Failed to create temporary file");
            std::io::copy(&mut content, &mut file)
                .expect(format!("Failed to download layer {}", &layer.digest).as_str());
            // extract the layer
            std::process::Command::new("tar")
                .current_dir("/bin")
                .args([
                    "-xf",
                    &layer_file_path.to_str().unwrap(),
                    "-C",
                    &path.to_str().unwrap(),
                ])
                .output()?;
        }
        layer_dir.close()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FsLayer {
    #[serde(rename(deserialize = "blobSum", serialize = "blobSum"))]
    pub digest: String,
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Manifest {
    #[serde(rename(deserialize = "fsLayers", serialize = "fsLayers"))]
    pub layers: Vec<FsLayer>,
    // #[serde(skip)]
    pub etag: Option<String>
}

impl Manifest {
    pub fn new() -> Self {
        Manifest { ..Default::default() }
    }
}

pub fn authenticate(image: &Image) -> Result<AuthResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let auth_response: AuthResponse = client
        .get(format!(
            "https://auth.docker.io/token?service=registry.docker.io&scope=repository:{}/{}:pull",
            image.repository, image.name
        ))
        .send()?
        .json()?;
    Ok(auth_response)
}


pub fn parse_image_reference(reference: &str) -> Result<Image, String> {
    let image_and_version: Vec<&str> = reference.split(":").collect();
    let version: Result<Option<&str>, &str> = match image_and_version.len() {
        1 => Ok(None),
        2 => Ok(Some(image_and_version[1])),
        _ => Err("Invalid image format"),
    };
    let version = version?.unwrap_or("latest");
    let tokens: Vec<&str> = image_and_version[0].split("/").collect();
    match tokens.len() {
        1 => Ok(Image {
            name: tokens[0].to_string(),
            reference: version.to_string(),
            ..Default::default()
        }),
        2 => Ok(Image {
            repository: tokens[0].to_string(),
            name: tokens[1].to_string(),
            reference: version.to_string(),
            ..Default::default()
        }),
        _ => Err(format!("Invalid image format {}", reference)),
    }
}
