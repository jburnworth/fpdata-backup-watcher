use postgres::{Client, NoTls, Error};
use std::io;
use std::fs;

fn main() {
    let mut client = Client::connect("postgresql://postgres:jam@localhost:5432/prodBackupsDBbbb", NoTls);
    println!("Connected");

    println!("Watching for backups");

    let fname = std::path::Path::new("/home/james/backups/17000-21009352412.zip");
    let file = fs::File::open(&fname).unwrap();

    let mut zippy = zip::ZipArchive::new(file).unwrap();

    for i in 0..zippy.len() {
        let file = zippy.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        // Only print files, not directories
        if ! file.name().ends_with('/') {
            println!("Found file: {}", outpath.display());
        }
    }
}
