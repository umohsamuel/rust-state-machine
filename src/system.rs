use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
	block_number: u32,
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	pub fn inc_nonce(&mut self, who: &String) {
		let current_nonce = self.nonce.get(who).unwrap_or(&0);
		self.nonce.insert(who.clone(), *current_nonce + 1);
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::new();
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
		system.inc_nonce(&String::from("alice"));
		let alice_nonce = system.nonce.get(&String::from("alice"));
		assert_eq!(alice_nonce, Some(&1));
		let bob_nonce = system.nonce.get(&String::from("bob"));
		assert_eq!(bob_nonce, None);
	}
}
