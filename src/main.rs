extern crate rustylnk;
extern crate clap;
extern crate seek_bufread;
use clap::{App, Arg, ArgMatches};
use seek_bufread::BufReader;
use rustylnk::lnkpkg::lnk::{Lnk};
use std::fs;

fn process_directory(directory: &str, options: &ArgMatches) {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_string = path.into_os_string().into_string().unwrap();
            if path_string.ends_with(".lnk"){
                process_file(&path_string, &options);
            }
        }
    }
}

fn process_file(filename: &str, options: &ArgMatches) -> bool {
    let mut file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            println!("Could not open file: {} [error: {}]", filename, error);
            return false;
        }
    };

    let mut reader = BufReader::new(file);

    let lnk = match Lnk::new(reader) {
        Ok(lnk) => lnk,
        Err(error) => {
            println!("Could not parse file: {} [error: {}]", filename, error);
            return false;
        }
    };

    println!("{:#?}",lnk);

    return true;
}

fn is_directory(source: &str)->bool{
    let metadata = fs::metadata(source).unwrap();

    let file_type = metadata.file_type();

    file_type.is_dir()
}

fn main() {
    let source_arg = Arg::with_name("source")
        .short("s")
        .long("source")
        .value_name("PATH")
        .help("The LNK file or folder with LNK files to parse.")
        .takes_value(true);

    let options = App::new("RusyLnk")
        .version("0.0.0")
        .author("Matthew Seyer <https://github.com/forensicmatt/RustyUsn>")
        .about("LNK Parser written in Rust.")
        .arg(source_arg)   // add the journal parameter
        .get_matches();

    let source = options.value_of("source").unwrap();

    if is_directory(source) {
        process_directory(source,&options);
    } else {
        process_file(source,&options);
    }
}
