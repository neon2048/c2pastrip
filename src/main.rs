use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use c2pa::{Error, Manifest, ManifestStore};
use clap::Parser;

fn main() -> ExitCode {
    /// Removes C2PA manifests from files
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// Remove C2PA manifests from <FILE>
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
        /// Place stripped output into <FILE>
        #[arg(short, long, value_name = "FILE")]
        out: Option<PathBuf>,
        /// Overwrite --out file if it already exists
        #[arg(long)]
        force: bool,
    }

    let args = Args::parse();

    let mut tgtfile = args.file;

    let mf_store = ManifestStore::from_file(&tgtfile);

    match mf_store {
        Ok(_m) => {}
        Err(e) => match e {
            Error::JumbfNotFound => {
                eprintln!("No C2PA manifest found");
                return ExitCode::from(1);
            }
            _ => {
                eprintln!("Error checking manifest: {}", e);
                return ExitCode::from(2);
            }
        },
    }

    if let Some(out) = args.out {
        if !args.force && Path::new(&out).exists() {
            eprintln!("Error: output file already exists, specify --force to overwrite");
            return ExitCode::from(2);
        }

        match fs::copy(&tgtfile, &out) {
            Ok(_x) => tgtfile = out,
            Err(e) => {
                eprintln!("Error writing output file: {}", e);
                return ExitCode::from(2);
            }
        }
    }

    let result = Manifest::remove_manifest(&tgtfile);

    match result {
        Ok(_x) => {
            return ExitCode::from(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return ExitCode::from(2);
        }
    }
}
