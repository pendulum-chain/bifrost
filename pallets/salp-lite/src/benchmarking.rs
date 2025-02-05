// This file is part of Bifrost.

// Copyright (C) 2019-2022 Liebi Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// Ensure we're `no_std` when compiling for Wasm.
#[cfg(feature = "runtime-benchmarks")]
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use node_primitives::ParaId;
use sp_runtime::{traits::Bounded, SaturatedConversion};
use sp_std::prelude::*;

pub use crate::{Pallet as Salp, *};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
	// compare to the last event record
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

fn create_fund<T: Config>(id: u32) -> ParaId {
	let cap = BalanceOf::<T>::max_value();
	let first_period = (0 as u32).into();
	let last_period = (7 as u32).into();
	let para_id = id;

	assert_ok!(Salp::<T>::create(RawOrigin::Root.into(), para_id, cap, first_period, last_period));

	para_id
}

#[allow(dead_code)]
fn contribute_fund<T: Config>(who: &T::AccountId, index: ParaId) {
	let value = T::MinContribution::get();
	let confirmer: T::RuntimeOrigin =
		RawOrigin::Signed(Salp::<T>::multisig_confirm_account().unwrap()).into();
	assert_ok!(Salp::<T>::issue(confirmer, who.clone(), index, value, [0; 32]));
}

benchmarks! {
	redeem {
		let fund_index = create_fund::<T>(1);
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin: T::RuntimeOrigin = RawOrigin::Signed(caller.clone()).into();
		let contribution = T::MinContribution::get();
		contribute_fund::<T>(&caller,fund_index);
		assert_ok!(Salp::<T>::fund_success(RawOrigin::Root.into(), fund_index));
		assert_ok!(Salp::<T>::fund_retire(RawOrigin::Root.into(), fund_index));
		assert_ok!(Salp::<T>::withdraw(RawOrigin::Root.into(), fund_index));
		assert_eq!(Salp::<T>::redeem_pool(), T::MinContribution::get());
	}: _(RawOrigin::Signed(caller.clone()), fund_index,contribution)
	verify {
		assert_eq!(Salp::<T>::redeem_pool(), 0_u32.saturated_into());
		assert_last_event::<T>(Event::<T>::Redeemed(caller.clone(), fund_index, (0 as u32).into(),(7 as u32).into(),contribution).into())
	}

	refund {
		let fund_index = create_fund::<T>(1);
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin: T::RuntimeOrigin = RawOrigin::Signed(caller.clone()).into();
		let contribution = T::MinContribution::get();
		contribute_fund::<T>(&caller,fund_index);
		assert_ok!(Salp::<T>::fund_fail(RawOrigin::Root.into(), fund_index));
		assert_ok!(Salp::<T>::withdraw(RawOrigin::Root.into(), fund_index));
		assert_eq!(Salp::<T>::redeem_pool(), T::MinContribution::get());
	}: _(RawOrigin::Signed(caller.clone()), fund_index,(0 as u32).into(),(7 as u32).into(),contribution)
	verify {
		assert_eq!(Salp::<T>::redeem_pool(), 0_u32.saturated_into());
		assert_last_event::<T>(Event::<T>::Refunded(caller.clone(), fund_index, (0 as u32).into(),(7 as u32).into(),contribution).into())
	}
}

impl_benchmark_test_suite!(Salp, crate::mock::new_test_ext(), crate::mock::Test);
