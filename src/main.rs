use flate2::read::GzDecoder;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;

fn main() {
    // create file
    let archive_file = File::open("archive.tar.gz").unwrap();
    // create archive
    let mut archive = Archive::new(GzDecoder::new(archive_file));
    let prefix = "/";

    println!("Extracted the following files: ");
    // Method 1
    archive
        .entries()
        .unwrap()
        .filter_map(|f| f.ok())
        .map(|mut entry| -> Result<PathBuf, std::io::Error> {
            let path = entry
                .path()
                .unwrap()
                // @todo - fix error: this works without the strip_prefix
                .strip_prefix(prefix)
                .unwrap()
                .to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    // Method 2
    // let itr = archive.entries().unwrap();
    // // filter files without errors
    // let filtered_files = itr.filter_map(|f| f.ok());

    // let list_files = filtered_files.map(|mut entry| -> Result<PathBuf, std::io::Error> {
    //     let path = entry.path().unwrap();
    //     let stripped_path = path.strip_prefix(prefix).unwrap(); // returns a pointer to the path
    //     let owned_path = stripped_path.to_owned();
    //     entry.unpack(&owned_path).unwrap();
    //     Ok(owned_path)
    // });

    // let _ = list_files
    //     .filter_map(|p| p.ok())
    //     .map(|path| println!("{}", path.display()));
}
