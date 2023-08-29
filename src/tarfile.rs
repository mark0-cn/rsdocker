use std::fs::File;
use std::path::Path;
use tar::Archive;

pub fn untar(tar_path: &String, dest_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(tar_path)?;
    let mut tar = Archive::new(file);
    let output_path = Path::new(dest_path);

    for entry in tar.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?;
        let output_entry_path = output_path.join(entry_path);

        if entry.header().entry_type().is_dir() {
            std::fs::create_dir_all(&output_entry_path)?;
        } else {
            let mut output_file = File::create(&output_entry_path)?;
            std::io::copy(&mut entry, &mut output_file)?;
        }
    }
    Ok(())
}
