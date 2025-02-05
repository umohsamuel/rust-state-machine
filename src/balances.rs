use crate::system;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: system::Config {
	type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);

		let to_balance = self.balance(&to);

		let new_caller_balance =
			caller_balance.checked_sub(&amount).ok_or("Insufficient balance")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Failed to complete")?;

		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::{system, Config, Pallet};

	struct TestConfig;

	impl Config for TestConfig {
		type Balance = u128;
	}

	impl system::Config for TestConfig {
		type AccountId = String;
		type Nonce = u32;
		type BlockNumber = u32;
	}

	#[test]
	fn init_balances() {
		let mut balances = Pallet::<TestConfig>::new();
		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 100),
			Err("Insufficient balance")
		);

		balances.set_balance(&"alice".to_string(), 100);

		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 50), Ok(()));

		assert_eq!(balances.balance(&"alice".to_string()), 50);
		assert_eq!(balances.balance(&"bob".to_string()), 50);
	}
}
