use postgres::{Client, NoTls, Error};
use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use serde::Deserialize;
use std::io::prelude::*;

fn main() {
    let mut client = Client::connect("postgresql://postgres:jam@localhost:5432/prodBackupsDBbbb", NoTls);
    println!("Connected");

    println!("Watching for backups");

    let fname = std::path::Path::new("/home/james/backups/17000-21009352412.zip");
    let file = File::open(&fname).unwrap();

    let mut zippy = zip::ZipArchive::new(file).unwrap();

    for i in 0..zippy.len() {
        let mut file = zippy.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        // Only print files, not directories
        if ! file.name().ends_with('/') {
            println!("Found file: {}", outpath.display());
            println!("{}", file.name());
            if file.name().ends_with(".json") {
                // Read file into string
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                println!("{}", contents);
                // parse string with serde
                backup_parsing::parse_json(file.name());
            }
        }
    }
}

mod backup_parsing {
    pub fn parse_json<P: AsRef<std::path::Path>>(path: P) {
        println!("Watch out, we're parsing jsons {:#?}", path.as_ref());
        // let me = std::fs::File::open(path);
        // let reader = std::io::BufReader::new(me);

    }
}