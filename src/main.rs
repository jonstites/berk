extern crate berk;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() -> std::io::Result<()> {
    let matches = App::new("berk")
        .version(crate_version!())
        .about("A git implementation for no good reason")
        .subcommand(SubCommand::with_name("hash-object").arg(Arg::with_name("file").index(1)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("hash-object") {
        if let Some(filename) = matches.value_of("file") {
            let object = berk::object_from_file(&filename)?;
            println!("{}", berk::hash_object(&berk::ObjectType::Blob, &object));
        }
    }
    Ok(())
}
