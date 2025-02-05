mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
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
	let _bal = runtime
		.balances
		.transfer(String::from("alice"), String::from("charlie"), 20)
		.map_err(|e| eprint!("{}", e));

	println!("{:#?}", runtime);
}
