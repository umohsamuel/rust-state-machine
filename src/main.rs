use crate::{balances::Config as BalancesConfig, system::Config};

mod balances;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type Nonce = u32;
	pub type BlockNumber = u32;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl Config for Runtime {
	type AccountId = types::AccountId;
	type Nonce = types::Nonce;
	type BlockNumber = types::BlockNumber;
}

impl BalancesConfig for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	runtime.balances.set_balance(&String::from("alice"), 100);

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	runtime.system.inc_nonce(&String::from("alice"));
	let _res = runtime
		.balances
		.transfer(String::from("alice"), String::from("bob"), 30)
		.map_err(|e| eprint!("{}", e));

	runtime.system.inc_nonce(&String::from("alice"));
	let _res = runtime
		.balances
		.transfer(String::from("alice"), String::from("charlie"), 20)
		.map_err(|e| eprint!("{}", e));

	println!("{:#?}", runtime);
}
