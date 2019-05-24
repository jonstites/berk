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
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("hash-object") {
        if let Some(filename) = matches.value_of("file") {
            let object = berk::object_from_file(&filename)?;
            let hash = berk::hash_object(&object);

            if matches.is_present("write") {
                let path = Path::new(".");
                let git_src = berk::find_git_src(&path)?;
                let object_dest = git_src
                    .join(".git/objects")
                    .join(hash[..2].to_string())
                    .join(hash[2..].to_string());

                if let Some(parent) = object_dest.parent() {
                    create_dir_all(parent)?;
                }

                let file = File::create(object_dest)?;
                let mut e = Encoder::new(file)?;

                std::io::copy(&mut &object.with_header()[..], &mut e)?;
                e.finish().into_result()?;
            }
            println!("{}", hash)
        }
    }
    Ok(())
}
