//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.
use clap::Parser;
use clap::Subcommand;
use alloy::{primitives::{address, Address, U256}, providers::ProviderBuilder, sol};
use alloy::transports::http::reqwest::Url;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "examples/abi/IWETH9.json"
);

#[derive(Parser)]
struct Cli {
	#[arg(short, long)]
	rpc_url: Url,
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	// Send a deposit event
	Deposit {
		/// contract address
		#[arg(short, long)]
		address: Address,
		#[arg(short, long)]
		amount: u64,
	},
}

#[tokio::main]
async fn main() {
	let cli = Cli::parse();

	let provider = ProviderBuilder::new().with_recommended_fillers().on_http(cli.rpc_url);

	match &cli.command {
		Some(Commands::Deposit { address, amount: _amount }) => {
			let contract = IWETH9::new(address.clone(), provider);
			contract.approve(address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), U256::from(1)).await;
		},
		_ => {
			eprintln!("Invalid command");
		},
	}
}
