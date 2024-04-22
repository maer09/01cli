mod cli;
mod process;
mod utils;

pub use cli::{Opts, SubCommand, Base64SubCommand, Base64Format, TextSubCommand, TextSignFormat};
pub use process::{process_csv, process_genpass, process_encode, process_decode};
pub use utils::*;