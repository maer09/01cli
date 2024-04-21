mod cli;
mod process;

pub use cli::{Opts, SubCommand, Base64SubCommand};
pub use process::{process_csv, process_genpass, process_encode, process_decode};