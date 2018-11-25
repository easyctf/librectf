use std::fs::{create_dir_all, File};
use std::io::{self, Write};

use failure::Error;
use migrations_internals::{
    mark_migrations_in_directory, run_migrations, search_for_migrations_directory,
};
use tempfile::TempDir;

use util::establish_connection;
use Config;
use Migrations;

#[derive(StructOpt)]
pub struct MigrateCommand {}

impl MigrateCommand {
    pub fn run(&self, config: &Config) -> Result<(), Error> {
        let dir = TempDir::new()?;

        // hack: write all the files and then run migrations
        // TODO: figure out how to construct migrations

        let migrations_dir = dir.path().join("migrations");
        for filename in Migrations::list() {
            let path = migrations_dir.join(filename);
            if let Some(parent) = path.parent() {
                create_dir_all(parent)?;
            }

            let mut file = File::create(&path)?;
            file.write_all(&Migrations::get(filename).unwrap())?;
            println!("{:?}", path);
        }

        let conn = establish_connection(config)
            .expect("Couldn't connect to database. Did you specify DATABASE_URL?");

        // run the migrations
        let migrations_dir = search_for_migrations_directory(dir.path())?;
        let migrations = mark_migrations_in_directory(&conn, &migrations_dir)?;

        for (migration, _) in migrations {
            run_migrations(&conn, vec![migration], &mut io::stdout())?;
        }

        Ok(())
    }
}
