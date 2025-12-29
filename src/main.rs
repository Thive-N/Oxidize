use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

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
        Command::Init { files } => {}
        Command::Decrypt {
            files,
            password,
            keep,
        } => {}
        Command::Encrypt {
            files,
            password,
            keep,
        } => {}
    }
}

fn read_password() -> String {
    rpassword::prompt_password("enter password> ").unwrap()
}
