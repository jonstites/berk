extern crate berk;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};
use std::path::Path;

use libflate::zlib::Encoder;
use std::fs::{create_dir_all, File};

fn main() -> berk::Result<()> {
    let matches = App::new("berk")
        .version(crate_version!())
        .about("A git implementation for no good reason")
        .subcommand(
            SubCommand::with_name("hash-object")
                .arg(Arg::with_name("file").index(1))
                .arg(
                    Arg::with_name("write")
                        .short("w")
                        .help("Actually write the object into the object database."),
                ),
        )
        .subcommand(
            SubCommand::with_name("init")
                .arg(Arg::with_name("directory")
                .index(1)
                .default_value(".")
                .help("If you provide a directory, the command is run inside it. If this directory does not exist, it will be created."))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("hash-object") {
        if let Some(filename) = matches.value_of("file") {
            let object = berk::object_from_file(&filename)?;
            let hash = berk::hash_object(&object);
            let hex_hash: String = hash.iter().map(|&byte| format!("{:02x}", byte)).collect();

            if matches.is_present("write") {
                let path = Path::new(".");
                let git_src = berk::find_git_src(&path)?;
                let object_dest = git_src
                    .join(".berk/objects")
                    .join(hex_hash[..2].to_string())
                    .join(hex_hash[2..].to_string());

                if let Some(parent) = object_dest.parent() {
                    create_dir_all(parent)?;
                }

                let file = File::create(object_dest)?;
                let mut e = Encoder::new(file)?;

                std::io::copy(&mut &object.with_header()[..], &mut e)?;
                e.finish().into_result()?;
            }
            println!("{}", hex_hash)
        }
    }
        if let Some(matches) = matches.subcommand_matches("init") {
            if let Some(directory) = matches.value_of("directory") {
                let path = Path::new(directory);
                berk::initialize_repo(path)?;
            }
        }

    
    Ok(())
}
