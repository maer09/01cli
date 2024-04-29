mod cli;
mod process;
mod utils;

pub use cli::{Opts, SubCommand, Base64SubCommand, Base64Format, TextSubCommand, TextSignFormat, HttpSubCommand};
pub use process::{process_csv, process_genpass, process_encode, process_decode, process_text_sign, process_text_verify, process_text_generate, process_http_serve};
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}