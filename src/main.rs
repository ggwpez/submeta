use array_bytes::hex2bytes;
use clap::{Parser, Subcommand, ValueEnum};
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use parity_scale_codec::Decode;

use std::io::Read;

#[derive(Parser, Debug)]
#[command(rename_all = "kebab-case")]
pub struct Cli {
	#[clap(subcommand)]
	pub cmd: Option<Command>,

	/// Expect bytes instead of hex.
	#[arg(long = "encoding", value_enum, default_value = "hex")]
	pub encoding: InputEncoding,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	Pallets(PalletsCmd),
}

#[derive(Parser, Debug)]
pub struct PalletsCmd {
	#[arg(long)]
	pub with_storage: bool,
	#[arg(long)]
	pub without_index: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum InputEncoding {
	Hex,
	Raw,
}

fn main() {
	let cli = Cli::parse();

	let mut buffer = Vec::new();
	std::io::stdin().read_to_end(&mut buffer).expect("Could not read from stdin.");
	let data = match cli.encoding {
		InputEncoding::Hex => {
			let mut buffer = String::from_utf8(buffer).expect("Could not parse stdin as UTF-8.");
			buffer = buffer.trim().to_string();

			if buffer.len() % 2 != 0 {
				panic!("Hex strings must have an even number of characters. Ensure that there is no newline at the end of the string.");
			}
			hex2bytes(&buffer).expect("Could not decode hex string.")
		},
		InputEncoding::Raw => buffer,
	};

	let metadata =
		RuntimeMetadataPrefixed::decode(&mut &data[..]).expect("Could not decode metadata.");
	let RuntimeMetadata::V14(metadata) = metadata.1 else {
        panic!("Only metadata version 14 is supported.");
    };

	match cli.cmd {
		Some(Command::Pallets(PalletsCmd { with_storage, without_index })) => {
			for (index, pallet) in metadata.pallets.iter().enumerate() {
				if without_index {
					println!("{}", pallet.name);
				} else {
					println!("{}: {}", index, pallet.name);
				}
				if with_storage {
					for (index, storage) in pallet
						.storage
						.clone()
						.map(|s| s.entries)
						.unwrap_or_default()
						.iter()
						.enumerate()
					{
						if without_index {
							println!("  {}", storage.name);
						} else {
							println!("  {}: {}", index, storage.name);
						}
					}
				}
			}
		},
		None => {
			println!("{:#?}", metadata);
		},
	};
}
