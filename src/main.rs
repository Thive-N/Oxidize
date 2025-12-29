use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

mod crypto;
mod oxide;

#[derive(Parser, Debug)]
#[command(name = "Oxidize", version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[arg(short, long, default_value = ".")]
        files: Vec<PathBuf>,
        #[arg(long, default_value_t = false)]
        force: bool,
    },
    /// decrypt the specified files
    Decrypt {
        /// files to decrypt, if empty, all files in the .oxide config will be decrypted
        #[arg(short, long)]
        files: Vec<PathBuf>,

        /// password to use for decryption, if not provided, will prompt for it
        #[arg(short, long, required = false)]
        password: Option<String>,

        /// keep the original files
        #[arg(short, long, default_value_t = false)]
        keep: bool,
    },
    /// encrypt the specified files
    Encrypt {
        /// files to encrypt, if empty, all files in the .oxide config will be decrypted
        #[arg(short, long)]
        files: Vec<PathBuf>,

        /// password to use for encryption, if not provided, will prompt for it
        #[arg(short, long, required = false)]
        password: Option<String>,

        /// keep the original files
        #[arg(short, long, default_value_t = false)]
        keep: bool,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Init { files, force } => {
            oxide::create_oxide_file(files, force).expect("Failed to create .oxide file")
        }
        Command::Decrypt {
            files,
            password,
            keep,
        } => crypto::decrypt(files, password, keep),
        Command::Encrypt {
            files,
            password,
            keep,
        } => {}
    }
}
