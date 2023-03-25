use std::path::PathBuf;

use clap::Parser;

/// A delightful debugger for a more civilized age :)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// The executable to deboogie
   #[arg(short, long)]
   pub executable: PathBuf,
}