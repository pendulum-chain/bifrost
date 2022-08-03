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
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for bifrost_slp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-03, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `git-actions`, CPU: `Intel(R) Xeon(R) CPU E5-26xx v4`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost_slp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./weight.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bifrost_slp.
pub trait WeightInfo {
	fn initialize_delegator() -> Weight;
	fn bond() -> Weight;
	fn bond_extra() -> Weight;
	fn rebond() -> Weight;
	fn delegate() -> Weight;
	fn redelegate() -> Weight;
	fn payout() -> Weight;
	fn liquidize() -> Weight;
	fn transfer_back() -> Weight;
	fn increase_token_pool() -> Weight;
	fn decrease_token_pool() -> Weight;
	fn update_ongoing_time_unit() -> Weight;
	fn refund_currency_due_unbond() -> Weight;
	fn charge_host_fee_and_tune_vtoken_exchange_rate() -> Weight;
	fn confirm_delegator_ledger_query_response() -> Weight;
	fn fail_delegator_ledger_query_response() -> Weight;
	fn set_xcm_dest_weight_and_fee() -> Weight;
	fn set_operate_origin() -> Weight;
	fn set_fee_source() -> Weight;
	fn add_delegator() -> Weight;
	fn remove_delegator() -> Weight;
	fn set_validators_by_delegator() -> Weight;
	fn set_delegator_ledger() -> Weight;
	fn add_validator() -> Weight;
	fn remove_validator() -> Weight;
	fn set_minimums_and_maximums() -> Weight;
	fn set_currency_delays() -> Weight;
	fn set_hosting_fees() -> Weight;
	fn set_currency_tune_exchange_rate_limit() -> Weight;
	fn set_ongoing_time_unit_update_interval() -> Weight;
	fn add_supplement_fee_account_to_whitelist() -> Weight;
	fn remove_supplement_fee_account_from_whitelist() -> Weight;
	fn unbond() -> Weight;
	fn unbond_all() -> Weight;
	fn undelegate() -> Weight;
	fn chill() -> Weight;
	fn transfer_to() -> Weight;
	fn supplement_fee_reserve() -> Weight;
	fn confirm_validators_by_delegator_query_response() -> Weight;
	fn fail_validators_by_delegator_query_response() -> Weight;

}

/// Weights for bifrost_slp using the Bifrost node and recommended hardware.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BifrostWeight<T> {
	// Storage: Slp DelegatorNextIndex (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Slp DelegatorsIndex2Multilocation (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:0 w:1)
	fn initialize_delegator() -> Weight {
		(123_178_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn bond() -> Weight {
		(305_409_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn bond_extra() -> Weight {
		(319_933_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn rebond() -> Weight {
		(308_003_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: Slp Validators (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp ValidatorsByDelegatorXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn delegate() -> Weight {
		(337_743_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: Slp Validators (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp ValidatorsByDelegatorXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn redelegate() -> Weight {
		(334_346_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn payout() -> Weight {
		(247_928_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn liquidize() -> Weight {
		(308_889_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn transfer_back() -> Weight {
		(225_916_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: VtokenMinting TokenPool (r:1 w:1)
	fn increase_token_pool() -> Weight {
		(77_203_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: VtokenMinting TokenPool (r:1 w:1)
	fn decrease_token_pool() -> Weight {
		(81_254_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp OngoingTimeUnitUpdateInterval (r:1 w:0)
	// Storage: Slp LastTimeUpdatedOngoingTimeUnit (r:1 w:1)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:1)
	fn update_ongoing_time_unit() -> Weight {
		(118_434_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Tokens Accounts (r:1 w:0)
	fn refund_currency_due_unbond() -> Weight {
		(42_412_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp CurrencyTuneExchangeRateLimit (r:1 w:0)
	// Storage: VtokenMinting TokenPool (r:1 w:1)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:0)
	// Storage: Slp CurrencyLatestTuneRecord (r:1 w:1)
	// Storage: Slp HostingFees (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp DelegatorLatestTuneRecord (r:1 w:1)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	fn charge_host_fee_and_tune_vtoken_exchange_rate() -> Weight {
		(295_413_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:1 w:1)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	// Storage: PolkadotXcm Queries (r:1 w:0)
	fn confirm_delegator_ledger_query_response() -> Weight {
		(172_661_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:1 w:1)
	// Storage: PolkadotXcm Queries (r:1 w:0)
	fn fail_delegator_ledger_query_response() -> Weight {
		(146_461_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp XcmDestWeightAndFee (r:1 w:1)
	fn set_xcm_dest_weight_and_fee() -> Weight {
		(85_688_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:1)
	fn set_operate_origin() -> Weight {
		(76_805_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp FeeSources (r:1 w:1)
	fn set_fee_source() -> Weight {
		(85_572_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp DelegatorsIndex2Multilocation (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:0 w:1)
	fn add_delegator() -> Weight {
		(94_198_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:1)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	// Storage: Slp DelegatorsIndex2Multilocation (r:0 w:1)
	fn remove_delegator() -> Weight {
		(139_275_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp Validators (r:1 w:0)
	// Storage: Slp ValidatorsByDelegator (r:0 w:1)
	fn set_validators_by_delegator() -> Weight {
		(146_860_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	fn set_delegator_ledger() -> Weight {
		(121_608_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp Validators (r:1 w:1)
	fn add_validator() -> Weight {
		(86_198_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp Validators (r:1 w:1)
	// Storage: Slp ValidatorsByDelegator (r:1 w:0)
	fn remove_validator() -> Weight {
		(111_965_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp MinimumsAndMaximums (r:1 w:1)
	fn set_minimums_and_maximums() -> Weight {
		(76_950_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp CurrencyDelays (r:1 w:1)
	fn set_currency_delays() -> Weight {
		(75_483_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp HostingFees (r:1 w:1)
	fn set_hosting_fees() -> Weight {
		(79_272_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp CurrencyTuneExchangeRateLimit (r:1 w:1)
	fn set_currency_tune_exchange_rate_limit() -> Weight {
		(79_829_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp OngoingTimeUnitUpdateInterval (r:1 w:1)
	// Storage: Slp LastTimeUpdatedOngoingTimeUnit (r:0 w:1)
	fn set_ongoing_time_unit_update_interval() -> Weight {
		(83_113_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Slp SupplementFeeAccountWhitelist (r:1 w:1)
	fn add_supplement_fee_account_to_whitelist() -> Weight {
		(87_896_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp SupplementFeeAccountWhitelist (r:1 w:1)
	fn remove_supplement_fee_account_from_whitelist() -> Weight {
		(93_144_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:0)
	// Storage: Slp CurrencyDelays (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn unbond() -> Weight {
		(346_707_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:0)
	// Storage: Slp CurrencyDelays (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn unbond_all() -> Weight {
		(334_251_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp ValidatorsByDelegator (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp ValidatorsByDelegatorXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn undelegate() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn chill() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn transfer_to() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn supplement_fee_reserve() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn confirm_validators_by_delegator_query_response() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn fail_validators_by_delegator_query_response() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Slp DelegatorNextIndex (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Slp DelegatorsIndex2Multilocation (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:0 w:1)
	fn initialize_delegator() -> Weight {
		(123_178_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn bond() -> Weight {
		(305_409_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn bond_extra() -> Weight {
		(319_933_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn rebond() -> Weight {
		(308_003_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: Slp Validators (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp ValidatorsByDelegatorXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn delegate() -> Weight {
		(337_743_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: Slp Validators (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp ValidatorsByDelegatorXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn redelegate() -> Weight {
		(334_346_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn payout() -> Weight {
		(247_928_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn liquidize() -> Weight {
		(308_889_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn transfer_back() -> Weight {
		(225_916_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: VtokenMinting TokenPool (r:1 w:1)
	fn increase_token_pool() -> Weight {
		(77_203_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: VtokenMinting TokenPool (r:1 w:1)
	fn decrease_token_pool() -> Weight {
		(81_254_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp OngoingTimeUnitUpdateInterval (r:1 w:0)
	// Storage: Slp LastTimeUpdatedOngoingTimeUnit (r:1 w:1)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:1)
	fn update_ongoing_time_unit() -> Weight {
		(118_434_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Tokens Accounts (r:1 w:0)
	fn refund_currency_due_unbond() -> Weight {
		(42_412_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp CurrencyTuneExchangeRateLimit (r:1 w:0)
	// Storage: VtokenMinting TokenPool (r:1 w:1)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:0)
	// Storage: Slp CurrencyLatestTuneRecord (r:1 w:1)
	// Storage: Slp HostingFees (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp DelegatorLatestTuneRecord (r:1 w:1)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	fn charge_host_fee_and_tune_vtoken_exchange_rate() -> Weight {
		(295_413_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:1 w:1)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	// Storage: PolkadotXcm Queries (r:1 w:0)
	fn confirm_delegator_ledger_query_response() -> Weight {
		(172_661_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:1 w:1)
	// Storage: PolkadotXcm Queries (r:1 w:0)
	fn fail_delegator_ledger_query_response() -> Weight {
		(146_461_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp XcmDestWeightAndFee (r:1 w:1)
	fn set_xcm_dest_weight_and_fee() -> Weight {
		(85_688_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:1)
	fn set_operate_origin() -> Weight {
		(76_805_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp FeeSources (r:1 w:1)
	fn set_fee_source() -> Weight {
		(85_572_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp DelegatorsIndex2Multilocation (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:0 w:1)
	fn add_delegator() -> Weight {
		(94_198_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:1)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	// Storage: Slp DelegatorsIndex2Multilocation (r:0 w:1)
	fn remove_delegator() -> Weight {
		(139_275_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp Validators (r:1 w:0)
	// Storage: Slp ValidatorsByDelegator (r:0 w:1)
	fn set_validators_by_delegator() -> Weight {
		(146_860_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:1)
	fn set_delegator_ledger() -> Weight {
		(121_608_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp Validators (r:1 w:1)
	fn add_validator() -> Weight {
		(86_198_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp Validators (r:1 w:1)
	// Storage: Slp ValidatorsByDelegator (r:1 w:0)
	fn remove_validator() -> Weight {
		(111_965_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp MinimumsAndMaximums (r:1 w:1)
	fn set_minimums_and_maximums() -> Weight {
		(76_950_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp CurrencyDelays (r:1 w:1)
	fn set_currency_delays() -> Weight {
		(75_483_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp HostingFees (r:1 w:1)
	fn set_hosting_fees() -> Weight {
		(79_272_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp CurrencyTuneExchangeRateLimit (r:1 w:1)
	fn set_currency_tune_exchange_rate_limit() -> Weight {
		(79_829_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp OngoingTimeUnitUpdateInterval (r:1 w:1)
	// Storage: Slp LastTimeUpdatedOngoingTimeUnit (r:0 w:1)
	fn set_ongoing_time_unit_update_interval() -> Weight {
		(83_113_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Slp SupplementFeeAccountWhitelist (r:1 w:1)
	fn add_supplement_fee_account_to_whitelist() -> Weight {
		(87_896_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp SupplementFeeAccountWhitelist (r:1 w:1)
	fn remove_supplement_fee_account_from_whitelist() -> Weight {
		(93_144_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp MinimumsAndMaximums (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:0)
	// Storage: Slp CurrencyDelays (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn unbond() -> Weight {
		(346_707_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(14 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:0)
	// Storage: Slp CurrencyDelays (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp DelegatorLedgerXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn unbond_all() -> Weight {
		(334_251_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Slp OperateOrigins (r:1 w:0)
	// Storage: Slp DelegatorLedgers (r:1 w:0)
	// Storage: Slp ValidatorsByDelegator (r:1 w:0)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: Slp DelegatorsMultilocation2Index (r:1 w:0)
	// Storage: Slp XcmDestWeightAndFee (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: Slp ValidatorsByDelegatorXcmUpdateQueue (r:0 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn undelegate() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}

	fn chill() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn transfer_to() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn supplement_fee_reserve() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn confirm_validators_by_delegator_query_response() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn fail_validators_by_delegator_query_response() -> Weight {
		(307_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
}
