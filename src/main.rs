extern crate berk;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};
use std::path::Path;

use crate::berk::Object;
use std::env;
use std::fs::create_dir_all;

extern crate chrono;

use chrono::Local;

fn main() -> berk::Result<()> {
    let matches = App::new("berk")
        .version(crate_version!())
        .about("A git implementation for no good reason")

        .subcommand(
            SubCommand::with_name("hash-object")
                .arg(Arg::with_name("file").index(1).required(true))
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
                .help("If you provide a directory, the command is run inside it. If this directory does not exist, it will be created."))
        )
        .subcommand(
            SubCommand::with_name("commit")
                .arg(Arg::with_name("message")
                .short("m")
                .long("message")
                .takes_value(true)
                .required(true))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        if let Some(directory) = matches.value_of("directory") {
            let path = Path::new(directory).canonicalize()?;
            let git_path = path.join(".git");

            let dirs = vec!["objects", "refs"];
            for dir in dirs {
                create_dir_all(git_path.join(dir))?
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("commit") {
        let cwd = env::current_dir()?;
        let git_dir = cwd.join(".git");
        let db_dir = git_dir.join("objects");

        let db = berk::ObjectDatabase::new(db_dir);

        let file_entries = cwd.read_dir()?;

        let mut entries = Vec::new();
        for file_entry in file_entries {
            let file_entry = file_entry?;
            let metadata = file_entry.metadata()?;
            if metadata.is_file() {
                let path = file_entry.path();
                let contents = std::fs::read(&path)?;
                let blob = berk::Blob::new(contents);
                db.write_object(&blob)?;
                entries.push((file_entry.file_name().into_string()?, blob.get_oid()));
            }
        }

        let tree = berk::Tree::new(entries);
        db.write_object(&tree)?;

        let author_name = env::var_os("GIT_AUTHOR_NAME")
            .map(|name| name.into_string().unwrap())
            .unwrap_or_else(|| "nobody".to_string());
        let author_email = env::var_os("GIT_AUTHOR_EMAIL")
            .map(|email| email.into_string().unwrap())
            .unwrap_or_else(|| "nobody@example.com".to_string());
        let timestamp = Local::now().format("%s %z").to_string();
        let message = matches.value_of("message").unwrap().to_string();

        let commit = berk::Commit::new(
            tree.get_oid(),
            author_name,
            author_email,
            timestamp,
            message,
        );
        db.write_object(&commit)?;
    }

    Ok(())
}
