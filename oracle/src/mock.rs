#![cfg(test)]

use super::*;

use primitives::H256;
use sr_primitives::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	weights::Weight,
	Perbill,
};
use support::{impl_outer_origin, parameter_types};

impl_outer_origin! {
	pub enum Origin for Test {}
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
}

type AccountId = u64;
type Key = u32;
type Value = u32;

pub type Timestamp = timestamp::Module<Test>;

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}

impl timestamp::Trait for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
}

pub struct MockOperatorProvider;

impl OperatorProvider<AccountId> for MockOperatorProvider {
	fn can_feed_data(who: &AccountId) -> bool {
		Self::operators().contains(who)
	}

	fn operators() -> Vec<AccountId> {
		vec![1, 2, 3]
	}
}

impl Trait for Test {
	type Event = ();
	type OnNewData = ();
	type OperatorProvider = MockOperatorProvider;
	type CombineData = DefaultCombineData<Self>;
	type Time = timestamp::Module<Self>;
	type Key = Key;
	type Value = Value;
}
pub type ModuleOracle = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> runtime_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
