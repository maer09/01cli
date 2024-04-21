// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, process_genpass, process_encode, process_decode, Opts, SubCommand, Base64SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                "output.json".into()
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.number, opts.symbol)?
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?
            }
        },
    }

    Ok(())
}

