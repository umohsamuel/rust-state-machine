use crate::support::{ DispatchResult};
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	pub fn _get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}
}

#[macros::call]
impl<T: Config> Pallet<T> {
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("this content is already claimed");
		}

		self.claims.insert(claim, caller);

		Ok(())
	}

	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.claims.get(&claim).ok_or("claim not found")?;
		if caller != *owner {
			return Err("this content is owned by someone else");
		};
		self.claims.remove(&claim);

		Ok(())
	}
}



#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type Nonce = u32;
		type BlockNumber = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut proof = super::Pallet::<TestConfig>::new();

		assert_eq!(proof.get_claim(&"Not exist claim"), None);
		assert_eq!(proof.create_claim("alice", "new claim"), Ok(()));
		assert_eq!(proof.create_claim("bob", "new claim"), Err("this content is already claimed"));
		assert_eq!(proof.revoke_claim("alice", "new claim"), Ok(()));
		assert_eq!(proof.create_claim("bob", "another claim"), Ok(()));
		assert_eq!(
			proof.revoke_claim("alice", "another claim"),
			Err("this content is owned by someone else")
		);
	}
}
