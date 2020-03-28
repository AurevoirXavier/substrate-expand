pub mod sp_runtime {
	pub type DispatchResult = Result<(), DispatchError>;

	pub trait Dispatchable {
		type Origin;
		type Trait;

		fn dispatch(self, origin: Self::Origin) -> DispatchResult;
	}

	pub enum DispatchError {
		Ops,
	}
}

pub mod frame_support {
	pub type CallableCallFor<A, T> = <A as Callable<T>>::Call;

	pub trait Callable<T> {
		type Call;
	}

	pub trait MemoryStoredValue<T> {
		fn get() -> Self;
		fn put(new_value: T);
	}

	pub trait SimpleStorageValue<T> {
		fn get() -> Self;
		fn put(new_value: T);
	}

	pub trait SpecializeData<Balance, Module> {
		fn get(&self) -> Balance;
		fn put(&mut self, new: Balance);
	}
}

pub mod frame_system {
	use super::*;

	use frame_support::MemoryStoredValue;

	pub trait Trait: Sized {
		type Origin;
		type Data;
	}

	trait Store {
		type Data;
	}

	pub struct MemoryStorage<T: Trait> {
		data: T::Data,
	}

	pub struct Module<T>(std::marker::PhantomData<T>);
	impl<T: Trait> Store for Module<T> {
		type Data = Data<T>;
	}
	impl<T: Trait> MemoryStoredValue<T::Data> for Module<T> {
		fn get() -> Self {
			unimplemented!()
		}
		fn put(new_value: T::Data) {
			unimplemented!()
		}
	}

	pub struct Data<T>(std::marker::PhantomData<T>);
}

pub mod pallet_instance {
	use super::*;

	use frame_support::{MemoryStoredValue, SpecializeData};

	pub trait Trait<I: Instance = DefaultInstance>: frame_system::Trait {
		type Data: SpecializeData<Self::Balance, I>;
		type DataStore: MemoryStoredValue<<Self as pallet_instance::Trait<I>>::Data>;
		type Balance;
	}

	pub trait Instance: 'static {
		const PREFIX: &'static str;
	}

	pub struct Module<T: Trait<I>, I: Instance = DefaultInstance>(std::marker::PhantomData<(T, I)>);
	impl<T: Trait<I>, I: Instance> Module<T, I> {
		fn get(origin: T::Origin) -> sp_runtime::DispatchResult {
			unimplemented!()
		}
		fn put(origin: T::Origin, new_value: T::Balance) -> sp_runtime::DispatchResult {
			unimplemented!()
		}
	}

	pub struct DefaultInstance;
	impl Instance for DefaultInstance {
		const PREFIX: &'static str = "Instance";
	}

	pub struct Instance1;
	impl Instance for Instance1 {
		const PREFIX: &'static str = "Instance1";
	}
	pub struct Instance2;
	impl Instance for Instance2 {
		const PREFIX: &'static str = "Instance2";
	}

	pub enum Call<T: Trait<I>, I: Instance = DefaultInstance> {
		#[allow(non_camel_case_types)]
		get(T::Origin),
		#[allow(non_camel_case_types)]
		put(T::Origin, T::Balance),
	}
	impl<T: Trait<I>, I: Instance> frame_support::Callable<T> for Module<T, I> {
		type Call = Call<T, I>;
	}
	impl<T: Trait<I>, I: Instance> sp_runtime::Dispatchable for Call<T, I> {
		type Trait = T;
		type Origin = T::Origin;

		fn dispatch(self, _origin: Self::Origin) -> sp_runtime::DispatchResult {
			match self {
				Call::get(dest) => <Module<T, I>>::get(dest),
				Call::put(dest, new_value) => <Module<T, I>>::put(dest, new_value),
			}
		}
	}
}

mod runtime {
	pub mod impls {
		use super::*;

		use frame_support::SpecializeData;

		pub struct Data<Balance> {
			pub instance1_value: Balance,
			pub instance2_value: Balance,
		}
		impl SpecializeData<Value, pallet_instance::Instance1> for Data<Value> {
			fn get(&self) -> Value {
				self.instance1_value
			}
			fn put(&mut self, new_value: Value) {
				self.instance1_value = new_value;
			}
		}
		impl SpecializeData<Value, pallet_instance::Instance2> for Data<Value> {
			fn get(&self) -> Value {
				self.instance2_value
			}
			fn put(&mut self, new_value: Value) {
				self.instance2_value = new_value;
			}
		}
	}

	pub use impls::Data;

	use super::*;

	pub type SimpleUser = u8;
	pub type Value = u8;

	pub type System = frame_system::Module<Runtime>;
	pub type Instance1 = pallet_instance::Module<Runtime, pallet_instance::Instance1>;
	pub type Instance2 = pallet_instance::Module<Runtime, pallet_instance::Instance2>;

	pub struct Runtime;
	impl frame_system::Trait for Runtime {
		type Origin = SimpleUser;
		type Data = impls::Data<Value>;
	}
	impl pallet_instance::Trait<pallet_instance::Instance1> for Runtime {
		type Data = impls::Data<Value>;
		type DataStore = System;
		type Balance = Value;
	}
	impl pallet_instance::Trait<pallet_instance::Instance2> for Runtime {
		type Data = impls::Data<Value>;
		type DataStore = System;
		type Balance = Value;
	}

	pub enum Call {
		Instance1(frame_support::CallableCallFor<Instance1, Runtime>),
		Instance2(frame_support::CallableCallFor<Instance2, Runtime>),
	}
	impl sp_runtime::Dispatchable for Call {
		type Origin = SimpleUser;
		type Trait = Call;

		fn dispatch(self, origin: Self::Origin) -> sp_runtime::DispatchResult {
			match self {
				Call::Instance1(call) => call.dispatch(origin),
				Call::Instance2(call) => call.dispatch(origin),
			}
		}
	}
}

fn main() {}
