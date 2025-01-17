// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_bridge_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-12, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-itmxxexx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bridge-hub-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_bridge_messages
// --chain=bridge-hub-rococo-dev
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_messages`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_messages::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `538`
		//  Estimated: `52645`
		// Minimum execution time: 39_918_000 picoseconds.
		Weight::from_parts(40_996_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_two_messages_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `538`
		//  Estimated: `52645`
		// Minimum execution time: 50_245_000 picoseconds.
		Weight::from_parts(51_441_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `538`
		//  Estimated: `52645`
		// Minimum execution time: 44_572_000 picoseconds.
		Weight::from_parts(45_629_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_1_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `506`
		//  Estimated: `52645`
		// Minimum execution time: 38_804_000 picoseconds.
		Weight::from_parts(39_516_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_16_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `506`
		//  Estimated: `52645`
		// Minimum execution time: 68_674_000 picoseconds.
		Weight::from_parts(72_690_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `3878`
		// Minimum execution time: 32_833_000 picoseconds.
		Weight::from_parts(33_442_000, 0)
			.saturating_add(Weight::from_parts(0, 3878))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `3878`
		// Minimum execution time: 32_528_000 picoseconds.
		Weight::from_parts(33_627_000, 0)
			.saturating_add(Weight::from_parts(0, 3878))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:2 w:2)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `6086`
		// Minimum execution time: 37_493_000 picoseconds.
		Weight::from_parts(38_324_000, 0)
			.saturating_add(Weight::from_parts(0, 6086))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `XcmpQueue::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelevantMessagingState` (r:1 w:0)
	/// Proof: `ParachainSystem::RelevantMessagingState` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpMessages` (r:0 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpMessages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[128, 2048]`.
	fn receive_single_message_proof_with_dispatch(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `669`
		//  Estimated: `52645`
		// Minimum execution time: 64_104_000 picoseconds.
		Weight::from_parts(66_006_268, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			// Standard Error: 91
			.saturating_add(Weight::from_parts(7_932, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
