pub struct Block<Header, Extrinsic> {
	pub header: Header,
	pub extrinsics: Vec<Extrinsic>,
}

/// We are using an extremely simplified header which only contains the current block number.
/// On a real blockchain, you would expect to also find:
/// - parent block hash
/// - state root
/// - extrinsics root
/// - etc...
pub struct Header<BlockNumber> {
	pub block_number: BlockNumber,
}

pub struct Extrinsic<Caller, Call> {
	pub caller: Caller,
	pub call: Call,
}

pub type DispatchResult = Result<(), &'static str>;

pub trait Dispatch {
	type Caller;
	type Call;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
