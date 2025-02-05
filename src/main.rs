mod balances;
mod system;

// This is our main Runtime.
// It accumulates all the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	/* TODO:
		- Create a field `system` which is of type `system::Pallet`.
		- Create a field `balances` which is of type `balances::Pallet`.
	*/
	system: system::Pallet,
	balances: balances::Pallet,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	/* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
	/* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */

	// start emulating a block
	/* TODO: Increment the block number in system. */
	/* TODO: Assert the block number is what we expect. */

	// first transaction
	/* TODO: Increment the nonce of `alice`. */
	/* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
		- The transfer _could_ return an error. We should use `map_err` to print
		  the error if there is one.
		- We should capture the result of the transfer in an unused variable like `_res`.
	*/

	// second transaction
	/* TODO: Increment the nonce of `alice` again. */
	/* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */

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
