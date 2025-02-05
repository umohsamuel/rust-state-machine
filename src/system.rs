use num::traits::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
	type AccountId: Ord + Clone;
	type Nonce: Copy + Zero + One;
	type BlockNumber: Zero + Copy + One + AddAssign;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let current_nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		self.nonce.insert(who.clone(), current_nonce + T::Nonce::one());
	}
}

#[cfg(test)]
mod test {
	use super::Config;
	struct TestConfig;
	impl Config for TestConfig {
		type AccountId = String;
		type Nonce = u32;
		type BlockNumber = u32;
	}

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
		system.inc_nonce(&String::from("alice"));
		let alice_nonce = system.nonce.get(&String::from("alice"));
		assert_eq!(alice_nonce, Some(&1));
		let bob_nonce = system.nonce.get(&String::from("bob"));
		assert_eq!(bob_nonce, None);
	}
}
